/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
--- Procedures
drop function if exists find_distribution;
drop function if exists search_distributions;
drop function if exists find_vpin_audit;
---------------
-- DISTRIBUTION
---------------
CREATE OR REPLACE FUNCTION 
	insert_distribution(param_distribution ltree) 
RETURNS INTEGER AS 
$$ DECLARE 
	package_name text := ltree2text(subpath($1, 0, 1));
	sz INTEGER := nlevel($1);
	version_ltree LTREE := subpath(full_pkg_ltree, 1);
BEGIN 
	IF NOT EXISTS(
		select
			name
		FROM package p
		WHERE
			p.name = package_name
		) THEN 
			RAISE EXCEPTION 'No package exists named %',package_name;
	END IF;
	IF sz < 2 THEN 
		RAISE EXCEPTION 'Malformed distribution. %',param_distribution;
	END IF;
	INSERT INTO 
		distribution (package, version)
	VALUES
		(package_name, version_ltree);
	RETURN 1;
END $$ LANGUAGE plpgsql;

---------------
-- DISTRIBUTION
---------------
CREATE OR REPLACE FUNCTION 
	insert_distribution(param_distribution text) 
RETURNS INTEGER AS $$ 
DECLARE 
	full_pkg_ltree LTREE := text2ltree(lower(replace(replace ($1, '-', '.'), ' ', '')));
	package_name /*package.name%TYPE*/text := ltree2text(subpath(full_pkg_ltree, 0, 1));
	sz INTEGER := nlevel(full_pkg_ltree);
	pkg_ltree LTREE := subpath(full_pkg_ltree, 1);
BEGIN 
	IF NOT EXISTS
	(
		SELECT
			name
		FROM 
			package p
		WHERE
			p.name = package_name
	) 
	THEN 
		RAISE EXCEPTION 'No package exists named %',package_name;
	END IF;
	IF sz < 2 THEN 
		RAISE EXCEPTION 'Malformed distribution. %',param_distribution;
	END IF;
	INSERT INTO 
		distribution (package, version)
	VALUES
		(package_name :: text, pkg_ltree);
	RETURN 1;
END $$ LANGUAGE plpgsql;

--------------
--   ROLE   --
--------------
CREATE OR REPLACE FUNCTION 
	insert_role(param_role text) 
RETURNS INTEGER AS $$ 
DECLARE 
	role_ltree LTREE := text2ltree('any.' || lower(replace(replace ($1, '_', '.'), ' ', '')));
	sz INTEGER := nlevel(role_ltree);
BEGIN 
	for cnt in 1..sz LOOP
		INSERT INTO role (path)
		VALUES
			(subpath(role_ltree, 0, cnt)) 
		ON CONFLICT (path) 
			DO NOTHING;
	END LOOP;
	return 1;
END $$ LANGUAGE plpgsql;
--------------
--   SITE   --
--------------
CREATE OR REPLACE FUNCTION 
	insert_site(param_site text) 
RETURNS INTEGER AS $$ 
DECLARE 
	site_ltree LTREE := text2ltree('any.' || lower(replace(replace ($1, '_', '.'), ' ', '')));
	sz INTEGER := nlevel(site_ltree);
BEGIN 
	IF sz > 2 THEN 
		RAISE EXCEPTION 'malformed site %',$1;
	END IF;
-- since it can only be 2 levels deep, and there is no notion of an intermediate site,
-- we can just go ahead and create it.
-- FOR cnt IN 1..sz  LOOP
-- 	INSERT INTO site (path ) VALUES (subpath(site_ltree, 0, cnt))
-- 	ON CONFLICT (path) DO NOTHING;
-- END LOOP;
	INSERT INTO site (path)
	VALUES
  		(site_ltree);
	return 1;
END $$
LANGUAGE plpgsql;
-------------
--  LEVEL  --
-------------
CREATE OR REPLACE FUNCTION 
	insert_level(param_level text) 
RETURNS INTEGER AS $$ 
DECLARE 
	level_ltree LTREE := text2ltree('facility.' || lower(replace(replace ($1, '_', '.'), ' ', '')));
	sz INTEGER := nlevel(level_ltree);
BEGIN 
	IF nlevel(level_ltree) < 2 THEN
		RAISE EXCEPTION 'invalid level specified %', $1;
	END IF;
	for cnt in 1..sz LOOP
		INSERT INTO 
			level (path)
		VALUES
  			(subpath(level_ltree, 0, cnt)) 
		ON CONFLICT (path) DO NOTHING;
	END LOOP;
	RETURN 1;
END $$ LANGUAGE plpgsql;

----------------
--  PLATFORM  --
----------------
CREATE OR REPLACE FUNCTION 
	insert_platform(param_platform text) 
RETURNS INTEGER AS $$ 
DECLARE 
	platform_ltree LTREE := text2ltree('any.' || lower(replace ($1, ' ', '_')));
	sz INTEGER := nlevel(platform_ltree);
BEGIN 
	IF nlevel(platform_ltree) < 2 THEN 
		RAISE EXCEPTION 'invalid platform specified %', $1;
	END IF;
	for cnt in 1..sz LOOP
		INSERT INTO 
			platform (path)
		VALUES
  			(subpath(platform_ltree, 0, cnt)) 
		on conflict (path) 
			do nothing;
	END LOOP;
	return 1;
END $$ LANGUAGE plpgsql;
-------------------------
--  insert_versionpin  --
-------------------------
CREATE OR REPLACE FUNCTION 
	insert_versionpin(
		distribution_n text,
		level_n text default 'facility',
		site_n text default 'any',
		role_n text default 'any',
		platform_n text default 'any'
	) 
RETURNS INTEGER AS $$ 
DECLARE 
	level_ltree ltree := 'facility';
	site_ltree ltree := 'any';
	role_ltree ltree := 'any';
	platform_ltree ltree := 'any';
	package_name /*package.name %TYPE*/ text := '';
	distribution_ltree ltree := '';
	distribution_version ltree;
	dist_id INTEGER;
	coord_id INTEGER;
BEGIN 
	IF lower(level_n) != 'facility' THEN 
		level_ltree = text2ltree('facility.' || lower(level_n));
	END IF;
	IF lower(site_n) != 'any' THEN 
		site_ltree = text2ltree('any.' || lower(site_n));
	END IF;
	IF lower(role_n) != 'any' THEN 
		role_ltree = text2ltree('any.' || lower(replace(role_n, '_', '.')));
	END IF;
	IF lower(platform_n) != 'any' THEN 
			platform_ltree = text2ltree('any.' || lower(platform_n));
	END IF;
	distribution_ltree = text2ltree(replace(lower(distribution_n), '-', '.'));
	package_name = ltree2text(subpath(distribution_ltree, 0, 1));
	distribution_version = subpath(distribution_ltree, 1);
	
	SELECT
		id 
	INTO 
		dist_id
	FROM 
		distribution
	WHERE
		distribution.package = package_name
		AND distribution.version = distribution_version;
		raise notice 'dist_id is % ',dist_id;
	IF dist_id IS NULL THEN 
		RAISE EXCEPTION 'unable to find distribtion for % %', package_name, distribution_version;
	END IF;

	insert into pkgcoord (role, level, platform, site, package) 
	values (role_ltree, level_ltree, platform_ltree, site_ltree, package_name)
	on conflict do nothing;
	
	SELECT
		id 
	INTO 
		coord_id
	FROM 
		pkgcoord
	WHERE
		pkgcoord.package = package_name
		AND pkgcoord.level = level_ltree 
		AND pkgcoord.role = role_ltree 
		AND pkgcoord.platform = platform_ltree
		AND pkgcoord.site = site_ltree;
		raise notice 'pkgcoord id is % ',coord_id;
	
	IF coord_id IS NULL THEN 
		RAISE EXCEPTION 'unable to find pkgcoord for % % % % %', package_name, level_n, role_n, platform_n, site_n;
	END IF;

	INSERT INTO versionpin (
		coord,
		distribution
	)
	VALUES
		(
		coord_id,
		dist_id
		) ;
	RETURN 1;
END $$ LANGUAGE plpgsql;

-------------------------
--  update_versionpin  --
-------------------------
CREATE OR REPLACE FUNCTION 
	update_versionpin(
		distribution_n text,
		level_n text default 'facility',
		site_n text default 'any',
		role_n text default 'any',
		platform_n text default 'any'
	) 
RETURNS INTEGER AS $$ 
DECLARE 
	level_ltree ltree := 'facility';
	site_ltree ltree := 'any';
	role_ltree ltree := 'any';
	platform_ltree ltree := 'any';
	package_name /*package.name %TYPE*/ text := '';
	distribution_ltree ltree := '';
	distribution_version ltree;
	dist_id INTEGER;
	coord_id INTEGER;
	vp_id INTEGER; -- versionpin
BEGIN 
	IF lower(level_n) != 'facility' THEN 
		level_ltree = text2ltree('facility.' || lower(level_n));
	END IF;
	IF lower(site_n) != 'any' THEN 
		site_ltree = text2ltree('any.' || lower(site_n));
	END IF;
	IF lower(role_n) != 'any' THEN 
		role_ltree = text2ltree('any.' || lower(replace(role_n, '_', '.')));
	END IF;
	IF lower(platform_n) != 'any' THEN 
			platform_ltree = text2ltree('any.' || lower(platform_n));
	END IF;
	distribution_ltree = text2ltree(replace(lower(distribution_n), '-', '.'));
	package_name = ltree2text(subpath(distribution_ltree, 0, 1));
	distribution_version = subpath(distribution_ltree, 1);
	
	SELECT
		id 
	INTO 
		coord_id
	FROM 
		pkgcoord
	WHERE
		pkgcoord.package = package_name
		AND pkgcoord.level = level_ltree 
		AND pkgcoord.role = role_ltree 
		AND pkgcoord.platform = platform_ltree
		AND pkgcoord.site = site_ltree;
		raise notice 'pkgcoord id is % ',coord_id;
	IF coord_id IS NULL THEN 
		RAISE EXCEPTION 'unable to find pkgcoord for % % % % %', package_name, level_n, role_n, platform_n, site_n;
	END IF;

	SELECT
		id 
	INTO 
		dist_id
	FROM 
		distribution
	WHERE
		distribution.package = package_name
		AND distribution.version = distribution_version;
		raise notice 'dist_id is % ',dist_id;
	IF dist_id IS NULL THEN 
		RAISE EXCEPTION 'unable to find distribtion for % %', package_name, distribution_version;
	END IF;

	select 
		id 
	into 
		vp_id
	from 
		versionpin 
	where 
		coord = coord_id
		and distribution in (
			select 
				id 
			from 
				distribution
			where 
				package=package_name
		);
	if vp_id is null then 
		raise exception 'uanble to find versionpin with supplied coord id % and package %',coord_id, package_name;
	end if;

	update  
		versionpin 
	set 
		distribution = dist_id 
	where 
		versionpin.id = vp_id;
		
	RETURN 1;
END $$ LANGUAGE plpgsql;

----------------------------
--  search_distributions  --
----------------------------
/*
Search for the set of distributions matching the supplied  context.

# Arguments
* `package_name` - The name f
*/
CREATE OR REPLACE FUNCTION 
	search_distributions(
	package_name text,
	level text default 'facility',
	site text default 'any',
	role text default 'any',
	platform text default 'any'
	) 
RETURNS TABLE(
  versionpin_id integer,
  distribution_id integer,
  pkgcoord_id integer,
  distribution text,
  package text,
  version ltree,
  level_name text,
  level_path ltree,
  site_name text,
  site_path ltree,
  role_name text,
  role_path ltree,
  platform_name text,
  platform_path ltree,
  withs text []
) AS $$ 
DECLARE 
	level_ltree ltree := '';
	site_ltree ltree := '';
	role_ltree ltree := '';
	platform_ltree ltree := '';
BEGIN 
	IF lower(level) = 'facility' THEN 
		level_ltree := text2ltree(lower(level));
	ELSE 
		level_ltree := text2ltree('facility.' || replace(lower(level), ' ', '_'));
	END IF;
	IF lower(site) = 'any' THEN 
		site_ltree := text2ltree(lower(site));
	ELSE 
		site_ltree := text2ltree('any.' || replace(lower(site), ' ', '_'));
	END IF;
	IF lower(role) = 'any' THEN 
		role_ltree := text2ltree('any');
	ELSE role_ltree := text2ltree('any.' || replace(lower(role), '_', '.'));
	END IF;
	IF lower(platform) = 'any' THEN 
		platform_ltree := text2ltree('any');
	ELSE 
		platform_ltree := text2ltree('any.' || replace(lower(platform), ' ', '_'));
	END IF;
	RETURN QUERY SELECT
		v.id,
		v.distribution_id,
		v.pkgcoord_id,
		v.distribution,
		v.package,
		v.version,
		v.level,
		v.level_path,
		v.site,
		v.site_path,
		v.role,
		v.role_path,
		v.platform,
		v.platform_path,
		v.withs
	FROM 
		versionpin_view 
	AS v
	WHERE
		v.package = package_name
		AND v.platform_path @> platform_ltree
		AND v.role_path @> role_ltree
		AND v.site_path @> site_ltree
		AND v.level_path @> level_ltree;
END $$ LANGUAGE plpgsql;
-------------------------
--  find_distribution  --
-------------------------
/*
Retrieve a specific distribution, given contextual
information, including level, site, role, and platform,
in addition to package name.

It should be noted that one only has to supply the package
name. All other parameters have reasonable, general defaults.
*/
CREATE OR REPLACE FUNCTION 
	find_distribution(
		package_name text,
		level text default 'facility',
		site text default 'any',
		role text default 'any',
		platform text default 'any'
	) 
RETURNS TABLE(
  versionpin_id integer,
  distribution_id integer, 
  pkgcoord_id integer,
  distribution text,
  package text,
  version ltree,
  level_name text,
  level_path ltree,
  site_name text,
  site_path ltree,
  role_name text,
  role_path ltree,
  platform_name text,
  platform_path ltree,
  withs text []
) AS $$ 
BEGIN 
	RETURN query
	SELECT
		*
	FROM 
		search_distributions(package_name, level, site, role, platform)
	ORDER BY
		level_path desc,
		site_path desc,
		role_path desc,
		platform_path desc
	LIMIT 1;
END $$ LANGUAGE plpgsql;
------------------------------
--  find_distribution_path  --
------------------------------
/*
return a path to a distribution on disk. Please note that you may
not get back what you expect. For instance, if you enter a non-extant
level, you may get back an ancestor. Eg if you supply dev01.rd but that
does not exist in the packrat database (because nobody has published to it)
you will get back dev01.
*/
CREATE OR REPLACE FUNCTION find_distribution_path(
    package_name text,
    level text default 'facility',
    site text default 'any',
    role text default 'any',
    platform text default 'any',
	login text default ''
)
RETURNS TEXT AS $$
DECLARE
	dist_p ltree;
	level_n text;
	platform_n text;
	version text;
BEGIN
	SELECT v.distribution_path,v.level_name,v.platform_name  INTO dist_p, level_n, platform_n
	FROM find_distribution(package_name, level, site, role, platform) AS v;
	version := ltree2text(subpath(dist_p,1));
	IF level_n = 'facility' THEN 
		RETURN '$DD_ROOT/tools/' || platform || '/package/' || package_name || '/' || version;
	ELSE
		IF login = '' THEN
			RETURN '$DD_SHOWS_ROOT/' || upper(REPLACE(level_n, '.', '/')) ||'/tools/' || platform || '/package/' || package_name || '/' || version;
		ELSE
			IF platform = 'any' THEN 
				RAISE EXCEPTION 'Must supply platform in addition to login for work paths to resolve';
			ELSE
				RETURN '$DD_SHOWS_ROOT/' || upper(REPLACE(level_n, '.', '/')) ||'/user/work.' || login || '/tools/' || platform || '/package/' || package_name || '/' || version;
			END IF;
		END IF;
	END IF;
END
$$ LANGUAGE plpgsql;
--------------------------
--  search_versionpins  --
--------------------------
/*
Return the list of all versionpin data that 
matches the version pin context (level, site, role, platform)
*/
CREATE OR REPLACE FUNCTION 
	search_versionpins(
		level text default 'facility',
		site text default 'any',
		role text default 'any',
		platform text default 'any'
	) 
RETURNS TABLE(
  id integer,
  package text,
  version ltree,
  distribution text,
  show text,
  level_name text,
  level_path ltree,
  site_name text,
  site_path ltree,
  role_name text,
  role_path ltree,
  platform_name text,
  platform_path ltree,
  withs text []
) AS $$ 
DECLARE 
	level_ltree ltree := '';
	site_ltree ltree := '';
	role_ltree ltree := '';
	platform_ltree ltree := '';
BEGIN 
	IF lower(level) = 'facility' THEN 
		level_ltree := text2ltree(lower(level));
	ELSE 
		level_ltree := text2ltree('facility.' || replace(lower(level), ' ', '_'));
	END IF;
	IF lower(site) = 'any' THEN 
		site_ltree := text2ltree(lower(site));
	ELSE 
		site_ltree := text2ltree('any.' || replace(lower(site), ' ', '_'));
	END IF;
	IF lower(role) = 'any' THEN role_ltree := text2ltree('any');
	ELSE 
		role_ltree := text2ltree('any.' || replace(lower(role), '_', '.'));
	END IF;
	IF lower(platform) = 'any' THEN 
		platform_ltree := text2ltree('any');
	ELSE 
		platform_ltree := text2ltree('any.' || replace(lower(platform), ' ', '_'));
	END IF;
RETURN QUERY SELECT
  v.id,
  v.package,
  v.version,
  v.distribution,
  v.show,
  v.level,
  v.level_path,
  v.site,
  v.site_path,
  v.role,
  v.role_path,
  v.platform,
  v.platform_path,
  v.withs
FROM (
    SELECT
      *
    FROM package
  ) AS pkg
INNER JOIN LATERAL (
    SELECT
      *
    FROM versionpin_view AS vp
    WHERE
      vp.package = pkg.name
      AND vp.platform_path @> platform_ltree
      AND vp.role_path @> role_ltree
      AND vp.site_path @> site_ltree
      AND vp.level_path @> level_ltree
    ORDER BY
      vp.level_path desc,
      vp.role_path desc,
      vp.platform_path desc,
      vp.site_path desc
    LIMIT
      1
  ) AS v ON true;
END $$ LANGUAGE plpgsql;
--------------------------
-- findall_versionpins  --
--------------------------
/*
findall_* searches in the oposite direction 
as search. looks for any children of the supplied parameters
*/
CREATE OR REPLACE FUNCTION 
	findall_versionpins(
		level text default 'facility',
		site text default 'any',
		role text default 'any',
		platform text default 'any',
		search_mode text default 'ancestor'
	) 
RETURNS TABLE(
  id integer,
  distribution_id INTEGER,
  pkgcoord_id INTEGER,
  package text,
  version ltree,
  distribution text,
  show text,
  level_name text,
  level_path ltree,
  site_name text,
  site_path ltree,
  role_name text,
  role_path ltree,
  platform_name text,
  platform_path ltree,
  withs text []
) AS $$ DECLARE 
	level_ltree ltree := '';
	site_ltree ltree := '';
	role_ltree ltree := '';
	platform_ltree ltree := '';
BEGIN 
	IF lower(level) = 'facility' THEN 
		level_ltree := text2ltree(lower(level));
	ELSE 
		level_ltree := text2ltree('facility.' || replace(lower(level), ' ', '_'));
	END IF;
	IF lower(site) = 'any' THEN 
		site_ltree := text2ltree(lower(site));
	ELSE 
		site_ltree := text2ltree('any.' || replace(lower(site), ' ', '_'));
	END IF;
	IF lower(role) = 'any' THEN 
		role_ltree := text2ltree('any');
	ELSE 
		role_ltree := text2ltree('any.' || replace(lower(role), '_', '.'));
	END IF;
	IF lower(platform) = 'any' THEN 
		platform_ltree := text2ltree('any');
	ELSE 
		platform_ltree := text2ltree('any.' || replace(lower(platform), ' ', '_'));
	END IF;
	IF (search_mode <> 'descendant' AND search_mode <> 'exact' AND search_mode <> 'ancestor') THEN
		RAISE EXCEPTION '% is invalid search mode', search_mode ;
	END IF;
RETURN QUERY
SELECT
  vp.id,
  vp.distribution_id,
  vp.pkgcoord_id,
  vp.package,
  vp.version,
  vp.distribution,
  vp.show,
  vp.level,
  vp.level_path,
  vp.site,
  vp.site_path,
  vp.role,
  vp.role_path,
  vp.platform,
  vp.platform_path,
  vp.withs
FROM versionpin_view AS vp
WHERE
	case when search_mode = 'exact' then
		vp.platform_path = platform_ltree
		AND vp.role_path = role_ltree
		AND vp.site_path = site_ltree
		AND vp.level_path = level_ltree
	when search_mode = 'ancestor' then
		vp.platform_path @> platform_ltree
		AND vp.role_path @> role_ltree
		AND vp.site_path @> site_ltree
		AND vp.level_path @> level_ltree
	ELSE 
		vp.platform_path <@ platform_ltree
		AND vp.role_path <@ role_ltree
		AND vp.site_path <@ site_ltree
		AND vp.level_path <@ level_ltree
	end
ORDER BY
  vp.level_path desc,
  vp.role_path desc,
  vp.platform_path desc,
  vp.site_path desc;
END $$ LANGUAGE plpgsql;
--------------------------------
--  debug_search_versionpins  --
--------------------------------
/*
Used to debug search_versionpins. This function exposes the inner query of
`search_versionpins`. Other than debugging, it has no real utility.
*/
CREATE OR REPLACE FUNCTION 
debug_search_versionpins(
  level text default 'facility',
  site text default 'any',
  role text default 'any',
  platform text default 'any',
  search_mode text default 'ancestor'
) RETURNS TABLE(
  versionpin_id integer,
  package text,
  version ltree,
  distribution text,
  show text,
  level_name text,
  level_path ltree,
  site_name text,
  site_path ltree,
  role_name text,
  role_path ltree,
  platform_name text,
  platform_path ltree,
  withs text []
) AS $$ 
DECLARE 
	level_ltree ltree := '';
	site_ltree ltree := '';
	role_ltree ltree := '';
	platform_ltree ltree := '';
	
BEGIN 
	IF lower(level) = 'facility' THEN 
		level_ltree := text2ltree(lower(level));
	ELSE 
		level_ltree := text2ltree('facility.' || replace(lower(level), ' ', '_'));
	END IF;
	IF lower(site) = 'any' THEN 
		site_ltree := text2ltree(lower(site));
	ELSE 
		site_ltree := text2ltree('any.' || replace(lower(site), ' ', '_'));
	END IF;
	IF lower(role) = 'any' THEN 
		role_ltree := text2ltree('any');
	ELSE 
		role_ltree := text2ltree('any.' || replace(lower(role), '_', '.'));
	END IF;
	IF lower(platform) = 'any' THEN 
		platform_ltree := text2ltree('any');
	ELSE 
		platform_ltree := text2ltree('any.' || replace(lower(platform), ' ', '_'));
	END IF;
	if (search_mode <> 'descendant' AND search_mode <> 'exact' AND search_mode <> 'ancestor') THEN
		RAISE EXCEPTION '% is invalid search mode', search_mode ;
	end if;
RETURN QUERY
SELECT
  vp.id,
  vp.package,
  vp.version,
  vp.distribution,
  vp.show,
  vp.level,
  vp.level_path,
  vp.site,
  vp.site_path,
  vp.role,
  vp.role_path,
  vp.platform,
  vp.platform_path,
  vp.withs
FROM 
	versionpin_view AS vp
WHERE
	case when search_mode = 'exact' then
  vp.platform_path = platform_ltree
  AND vp.role_path = role_ltree
  AND vp.site_path = site_ltree
  AND vp.level_path = level_ltree
  when search_mode = 'ancestor' then
  vp.platform_path @> platform_ltree
  AND vp.role_path @> role_ltree
  AND vp.site_path @> site_ltree
  AND vp.level_path @> level_ltree
  ELSE 
  vp.platform_path <@ platform_ltree
  AND vp.role_path <@ role_ltree
  AND vp.site_path <@ site_ltree
  AND vp.level_path <@ level_ltree
  end
ORDER BY
  vp.level_path desc,
  vp.role_path desc,
  vp.platform_path desc,
  vp.site_path desc;
END $$ LANGUAGE plpgsql;
/*

WHERE
	case when exact = 't' then
  vp.platform_path ~ platform_ltree
  AND vp.role_path ~ role_ltree
  AND vp.site_path ~ site_ltree
  AND vp.level_path ~ level_ltree
  else 
  vp.platform_path @> platform_ltree
  AND vp.role_path @> role_ltree
  AND vp.site_path @> site_ltree
  AND vp.level_path @> level_ltree
  end*/
------------------------------
--  find_distribution_deps  --
------------------------------
/*
 Helper function that unnests with array
*/
CREATE
OR REPLACE FUNCTION find_distribution_deps(
  package_name text,
  level text default 'facility',
  site text default 'any',
  role text default 'any',
  platform text default 'any'
) RETURNS TABLE(package_names text) AS $$ BEGIN RETURN query
SELECT
  unnest(vv.withs)
FROM (
    SELECT
      withs
    FROM search_distributions(package_name, level, site, role, platform) as v
    ORDER BY
      v.level_path desc,
      v.site_path desc,
      v.role_path desc,
      v.platform_path desc
    LIMIT
      1
  ) as vv;
END $$ LANGUAGE plpgsql;
-------------------------------
--  find_distribution_withs  --
-------------------------------
/*
Find the versions of a distribution's withs
*/
CREATE
OR REPLACE FUNCTION find_distribution_withs(
  package_name text,
  level text default 'facility',
  site text default 'any',
  role text default 'any',
  platform text default 'any'
) RETURNS TABLE(
	versionpin_id INTEGER,
	package text,
	version ltree,
	distribution text,
	show text,
	level_name text,
	level_path ltree,
	site_name text,
	site_path ltree,
	role_name text,
	role_path ltree,
	platform_name text,
	platform_path ltree,
	withs text []
) AS $$ 
BEGIN 
	RETURN QUERY 
	WITH packages AS (
		SELECT
			package_names
		FROM FIND_DISTRIBUTION_DEPS(package_name, level, site, role, platform)
	)
	SELECT
		srch.id,
		srch.package,
		srch.version,
		srch.distribution,
		srch.show,
		srch.level_name,
		srch.level_path,
		srch.site_name,
		srch.site_path,
		srch.role_name,
		srch.role_path,
		srch.platform_name,
		srch.platform_path,
		srch.withs
	FROM (
		SELECT
			*
		FROM SEARCH_VERSIONPINS(level, site, role, platform)
	) AS srch
	INNER JOIN 
		packages 
	ON 
		srch.package = packages.package_names;
END $$ LANGUAGE plpgsql;
/*
CREATE OR REPLACE FUNCTION append_with(
    package_name text,
    level text default 'facility',
    site text default 'any',
    role text default 'any',
    platform text default 'any'
)
RETURNS INTEGER AS $$
DECLARE
	vpin ltree; 
	pkg text;
BEGIN
	SELECT versionpin_ INTO vpin, pkg FROM 
END 
$$ LANGUAGE plpgsql;
*/

DROP FUNCTION IF EXISTS get_actions;
CREATE FUNCTION get_actions()
RETURNS TABLE (
	transaction_id bigint, 
	actions json
)
AS $$
BEGIN
  RETURN QUERY  
  SELECT
    la.transaction_id,
    json_build_object(table_name,
    case 
    when action = 'INSERT' THEN 
        json_build_object(action, ARRAY_AGG(row_data))
    when action = 'UPDATE' THEN
        json_build_object( action, ARRAY_AGG( json_build_object('from',to_json(row_data),'to',to_json(changed_fields))))
    WHEN action = 'DELETE' THEN 
        json_build_object(action, ARRAY_AGG(row_data))
    ELSE 
        json_build_object(action,table_name) 
    END 
    ) AS actions

FROM
    audit.logged_actions as la
GROUP BY
    la.transaction_id, la.action, la.table_name
ORDER BY
    la.transaction_id;
END
$$
LANGUAGE plpgsql;

-------------------------
--  findall_pkgcoords  --
-------------------------
CREATE OR REPLACE FUNCTION 
	findall_pkgcoords(
	package_name text default '%',
	level text default 'facility',
	role text default 'any',
	platform text default 'any',
	site text default 'any'
	) 
RETURNS TABLE(
  versionpin_id integer,
  package text,
  level_name text,
  level_path ltree,
  role_name text,
  role_path ltree,
  platform_name text,
  platform_path ltree,
  site_name text,
  site_path ltree
) AS $$ 
DECLARE 
	package_n text;
	level_ltree ltree := '';
	site_ltree ltree := '';
	role_ltree ltree := '';
	platform_ltree ltree := '';
BEGIN 
	package_n = lower(package);
	IF lower(level) = 'facility' THEN 
		level_ltree := text2ltree(lower(level));
	ELSE 
		level_ltree := text2ltree('facility.' || replace(lower(level), ' ', '_'));
	END IF;
	IF lower(role) = 'any' THEN 
		role_ltree := text2ltree('any');
	ELSE role_ltree := text2ltree('any.' || replace(lower(role), '_', '.'));
	END IF;
	IF lower(platform) = 'any' THEN 
		platform_ltree := text2ltree('any');
	ELSE 
		platform_ltree := text2ltree('any.' || replace(lower(platform), ' ', '_'));
	END IF;
	IF lower(site) = 'any' THEN 
		site_ltree := text2ltree(lower(site));
	ELSE 
		site_ltree := text2ltree('any.' || replace(lower(site), ' ', '_'));
	END IF;
	RETURN QUERY SELECT
		v.id as coord_id,
		v.package,
		v.level as level_name,
		v.level_path,
		v.role as role_name,
		v.role_path,
		v.platform as platform_name,
		v.platform_path,
		v.site as site_name,
		v.site_path
	FROM 
		pkgcoord_view 
	AS v
	WHERE
		v.package = package_name
		AND v.level_path @> level_ltree
		AND v.role_path @> role_ltree
		AND v.platform_path @> platform_ltree
		AND v.site_path @> site_ltree;
END $$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION 
	find_vpin_audit(
		tx_id BIGINT
	) 
RETURNS TABLE(
  id INTEGER,
  action text,
  transaction_id bigint,
  role_name text,
  level_name text,
  site_name text,
  platform_name text,
  package text,
  old text,
  new text
) AS $$ 
BEGIN 
	RETURN QUERY WITH auditvals AS 
    (
        SELECT 
            row_data->'id' AS id,
            row_data->'coord' AS coord, 
            row_data->'distribution' AS old_dist,
            audit.logged_actions.changed_fields->'distribution' AS new_dist,
            audit.logged_actions.transaction_id,
            audit.logged_actions.action
        FROM 
            audit.logged_actions
        WHERE 
            table_name='versionpin' AND 
            row_data is not null AND 
			audit.logged_actions.transaction_id = tx_id
    ) 
SELECT 
    auditvals.id::integer AS id,
    auditvals.action,
    auditvals.transaction_id::BIGINT,
    pkgcoord.role_name, 
    pkgcoord.level_name,
    pkgcoord.site_name,
    pkgcoord.platform_name,
	pkgcoord.package,
    CASE 
        WHEN auditvals.action = 'UPDATE' THEN distribution.name
        ELSE ''
    END AS old,
    distribution2.name AS new
FROM 
    pkgcoord_view as pkgcoord
INNER JOIN 
    auditvals 
ON 
    auditvals.coord::integer = pkgcoord.pkgcoord_id 
INNER JOIN 
    distribution_view AS distribution 
ON 
    auditvals.old_dist::INTEGER = distribution.distribution_id
INNER JOIN 
    distribution_view as distribution2
ON 
   CASE WHEN auditvals.action = 'UPDATE' 
   THEN 
        auditvals.new_dist::INTEGER 
    ELSE
        auditvals.old_dist::INTEGER
    END = distribution2.distribution_id;
END $$ LANGUAGE plpgsql;