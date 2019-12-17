CREATE EXTENSION IF NOT EXISTS ltree;
DROP SEQUENCE IF EXISTS pkgcoord_id_seq CASCADE;
DROP SEQUENCE IF EXISTS versionpin_id_seq CASCADE;

DROP SEQUENCE IF EXISTS revision_id_seq CASCADE;
DROP SEQUENCE IF EXISTS withpackage_id_seq CASCADE;
DROP TABLE IF EXISTS revision CASCADE; 

DROP TABLE IF EXISTS pkgcoord CASCADE;

DROP VIEW IF EXISTS versionpin_view CASCADE;
DROP TABLE IF EXISTS versionpin CASCADE;
DROP TABLE IF EXISTS package CASCADE;
DROP TABLE IF EXISTS distribution CASCADE;
DROP VIEW IF EXISTS role_view CASCADE;
DROP TABLE IF EXISTS role CASCADE;
DROP VIEW IF EXISTS platform_view CASCADE;
DROP TABLE IF EXISTS platform CASCADE;
DROP VIEW IF EXISTS level_view CASCADE;
DROP TABLE IF EXISTS level CASCADE;
DROP VIEW IF EXISTS site_view CASCADE;
DROP TABLE IF EXISTS site CASCADE;
DROP TABLE IF EXISTS changeset CASCADE;
DROP TABLE IF EXISTS withpackage CASCADE;

-------------
--  SITE   --
-------------
CREATE TABLE IF NOT EXISTS site (
	path LTREE DEFAULT 'any' PRIMARY KEY 
);
CREATE INDEX IF NOT EXISTS site_path_gist_idex ON site USING GIST (path);

CREATE OR REPLACE VIEW site_view AS (
    SELECT 
	path,
	CASE WHEN nlevel(path) = 2 THEN
	   ltree2text(subpath(path,1))	
	WHEN nlevel(path) > 2 THEN	
	    replace(
		ltree2text( 
			subpath(path,1)
		), '.','_'
	    )
	ELSE 'any' END	
	AS name 
	FROM site
);

-----------
-- LEVEL --
-----------
CREATE TABLE IF NOT EXISTS level (
	path LTREE DEFAULT 'facility' PRIMARY KEY
);

CREATE INDEX IF NOT EXISTS level_path_gist_idx ON level USING GIST (path);

CREATE OR REPLACE VIEW level_view AS (
    SELECT 
	path,
	CASE WHEN nlevel(path) = 2 THEN
	   ltree2text(subpath(path,1))	
	WHEN nlevel(path) > 2 THEN	
		ltree2text( 
			subltree(path,1,2)
	    )
	ELSE 'facility' END	
	AS show,
        CASE WHEN nlevel(path) > 1 THEN
	  ltree2text(subpath(path, 1))
        ELSE 'facility' END
	AS name 	
	FROM level
);

----------------
--  PLATFORM  --
----------------
CREATE TABLE IF NOT EXISTS platform (
	path LTREE DEFAULT 'any' PRIMARY KEY
);

CREATE INDEX IF NOT EXISTS platform_path_gist_idx ON platform USING GIST (path);

CREATE OR REPLACE VIEW platform_view AS (
    SELECT 
	path,
	CASE WHEN nlevel(path) = 2 THEN
	   ltree2text(subpath(path,1))	
	WHEN nlevel(path) > 2 THEN	
	    replace(
		ltree2text( 
			subpath(path,1)
		), '.','_'
	    )
	ELSE 'any' END	
	AS name 
	FROM platform
);

-----------
-- ROLE  --
-----------
CREATE TABLE IF NOT EXISTS role (
	path LTREE DEFAULT 'any' PRIMARY KEY 
);

CREATE INDEX IF NOT EXISTS role_path_gist_idx ON role USING GIST(path);

CREATE OR REPLACE VIEW role_view AS (
    SELECT 
	path,
	CASE WHEN nlevel(path) > 1 THEN	
	    replace(
		ltree2text( 
			subpath(path,1)
		), '.','_'
	    )
	ELSE 'any' END	
	AS name,
	CASE WHEN nlevel(path) = 2 THEN 'role'
	 WHEN nlevel(path) > 2 THEN 'subrole'
	ELSE 'any' END AS category
	FROM role
);

---------------
--  PACKAGE  --
---------------
-- should we ultimately merge this back in with the distribution?
CREATE TABLE IF NOT EXISTS package (
  name TEXT PRIMARY KEY
);

------------------
-- DISTRIBUTION --
------------------
CREATE TABLE IF NOT EXISTS distribution (
	id SERIAL PRIMARY KEY, 
	package TEXT REFERENCES package(name) NOT NULL,
	version LTREE NOT NULL,
	UNIQUE(package, version)
);

CREATE INDEX IF NOT EXISTS distribution_version_gist_idx ON distribution USING GIST(version);

CREATE OR REPLACE VIEW distribution_view AS (
	SELECT 
	id AS distribution_id,
	package,
	version,
	ltree2text(version) as version_name,
	package || '-' || ltree2text(version) AS name
	FROM 
	  distribution 
);

---------------
--  REVISION --
---------------
CREATE TABLE IF NOT EXISTS revision (
	id SERIAL PRIMARY KEY,
	author TEXT NOT NULL,
	comment text NOT NULL
);

------------------
-- PKGCOORD    --
------------------
CREATE TABLE IF NOT EXISTS pkgcoord (
	id SERIAL PRIMARY KEY,
	role LTREE DEFAULT 'any' REFERENCES role(path) NOT NULL,
	level LTREE DEFAULT 'facility' REFERENCES level(path) NOT NULL,
	site LTREE DEFAULT 'any' REFERENCES site(path) NOT NULL, 
	platform LTREE DEFAULT 'any' REFERENCES platform(path) NOT NULL, 
	package text references package(name) not null,
	UNIQUE (role, level, site, platform, package)
);


CREATE OR REPLACE VIEW pkgcoord_view AS (
	SELECT 
		id AS pkgcoord_id,
		package,
		level,
		level_view.name as level_name, 
		role,
		role_view.name as role_name, 
		platform,
		platform_view.name as platform_name,
		site,
		site_view.name as site_name
	FROM 
	  pkgcoord, level_view, role_view, platform_view, site_view
	WHERE 
		level_view.path = level  
		AND role_view.path = role
		AND platform_view.path = platform
		AND site_view.path = site

);
------------------
--  versionpin  --
------------------
CREATE TABLE IF NOT EXISTS versionpin (
	id SERIAL PRIMARY KEY,
	coord integer references pkgcoord(id) not null,
	distribution integer references distribution(id) not null,
	unique(coord,distribution)
);

CREATE OR REPLACE FUNCTION valid_package_in_versionpin()
RETURNS TRIGGER
AS $$
DECLARE
	existing_dist integer;
	dist_package text;
	coord_package text;
BEGIN
	if  (TG_OP = 'INSERT') then
	/*
	if pkgcoord.package 
	select versionpin where versionpin.coord = NEW.coord and versionpin.distribution in (select distribution where distribution.package = dist_package)
	*/
		
        select package from distribution into dist_package where distribution.id = NEW.distribution;
		select package from pkgcoord into coord_package where pkgcoord.id = new.coord; 
		if dist_package <> coord_package then 
		    RAISE EXCEPTION 'pkgcoord (% %) and distribution (% %) must have same package',
			NEW.id, coord_package, NEW.distribution, dist_package;
		end if;
		if exists(
				select 
					id 
				from 
					versionpin_view 
				where 
					coord_id = new.coord 
					and distribution_id in (
						select 
							id 
						from 
							distribution 
						where 
							package=coord_package
					)
		) then
			RAISE EXCEPTION 'pkgcoord % for package % exists', new.coord, dist_package;
		end if;
    end if;

     --code for update
     if  (TG_OP = 'UPDATE') then
	 	if OLD.coord <> NEW.coord then 
			RAISE EXCEPTION 'cannot update pkgcoord';
    	END IF;
        if OLD.distribution <> NEW.distribution then
			select package from distribution into dist_package where distribution.id = NEW.distribution;
			select package from pkgcoord into coord_package where pkgcoord.id = new.id; 
			if dist_package <> coord_package then 
				RAISE EXCEPTION 'pincooord and distribution must have same package';
			end if;
			
        end if;
     end if;
    RETURN NEW;
END
$$ LANGUAGE plpgsql;
--SQL;

DROP TRIGGER IF EXISTS update_or_insert_versionpin_trigger on versionpin;
CREATE TRIGGER update_or_insert_versionpin_trigger
    BEFORE INSERT OR UPDATE ON versionpin
    FOR EACH ROW 
    EXECUTE PROCEDURE valid_package_in_versionpin();

-----------------
-- WITHPACKAGE --
-----------------
/*
Was thinking of changing the name to dependency
*/
CREATE TABLE IF NOT EXISTS withpackage (
	id SERIAL PRIMARY KEY,
	-- the id of the versionpin this with relates to
	versionpin INTEGER REFERENCES versionpin(id) NOT NULL,
	package TEXT REFERENCES package(name) NOT NULL, 
	-- the order of the package in the list of withs
	pinorder INTEGER NOT NULL,
	UNIQUE (versionpin, package),
	UNIQUE(versionpin, pinorder)
);

CREATE OR REPLACE VIEW versionpin_withs AS (
	WITH cte AS (
		SELECT 
			versionpin,
			ARRAY_AGG(package) 
		AS withs 
		FROM (
			SELECT * 
			FROM 
				withpackage 
			ORDER BY 
				pinorder
			) AS v  
		GROUP BY 
			versionpin 
		ORDER BY 
			versionpin
	) 
	SELECT 
		vpn.id as versionpin_id,
		pc.role,
		pc.level,
		pc.site,
		pc.platform,
		pc.package,
		vpn.distribution as distribution_id,
		cte.withs 
	FROM 
		versionpin AS vpn 
	join pkgcoord as pc on vpn.coord = pc.id
	LEFT OUTER JOIN  
		cte 
	ON 
		cte.versionpin = vpn.id
);

CREATE OR REPLACE VIEW versionpin_view AS (
	select 
	versionpin.id as id,
	level_view.name AS level,
	pkgcoord.level AS level_path,
	level_view.show AS show,
	role_view.name AS role,
	pkgcoord.role AS role_path,
	site_view.name AS site,
	pkgcoord.id as pkgcoord_id,
	pkgcoord.site AS site_path, 
	platform_view.name AS platform,
	pkgcoord.platform AS platform_path,
	versionpin.coord as coord_id,
	versionpin.distribution as distribution_id,
	distribution_view.package,
	distribution_view.name AS distribution_name,
	distribution_view.version AS version,
	distribution_view.package || '-' || ltree2text(distribution_view.version) AS distribution,
	versionpin_withs.withs AS withs
	FROM versionpin, pkgcoord, role_view, level_view, site_view, platform_view, versionpin_withs, distribution_view
	WHERE 
		pkgcoord.id = versionpin.coord AND
	    level_view.path = versionpin_withs.level AND
	    role_view.path  = versionpin_withs.role AND
	    site_view.path = versionpin_withs.site AND
		versionpin.id = versionpin_withs.versionpin_id AND
	    platform_view.path = versionpin_withs.platform AND
	    --distribution_view.package = versionpin_withs.package AND
	    distribution_view.distribution_id = versionpin_withs.distribution_id
);

-----------------
--  CHANGESET  --
-----------------
/*
changeset is going to be a jsonb document to match what it is currently
{_schema, actions:[
	{'delete': {}}
]}
*/
-- CREATE TABLE IF NOT EXISTS changeset (
-- 	id SERIAL PRIMARY KEY,
-- 	revision INTEGER REFERENCES revision (id) NOT NULL,
-- 	versionpin INTEGER REFERENCES versionpin (id) NOT NULL,
-- 	UNIQUE (revision, versionpin)
-- );

---------------------------
-- basic initialization
--------------------------
INSERT INTO site (path) VALUES ('any');
INSERT INTO level (path) VALUES ('facility');
INSERT INTO platform (path) VALUES ('any');
INSERT INTO role (path) VALUES ('any');

