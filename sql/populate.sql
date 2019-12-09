
TRUNCATE site CASCADE;
TRUNCATE level CASCADE;
TRUNCATE platform CASCADE;
TRUNCATE role CASCADE;
TRUNCATE package CASCADE;
TRUNCATE distribution CASCADE;
TRUNCATE withpackage CASCADE;
TRUNCATE versionpin CASCADE;

---------------------------
-- basic initialization
--------------------------
INSERT INTO site
	(path)
VALUES
	('any');
INSERT INTO level
	(path)
VALUES
	('facility');
INSERT INTO platform
	(path)
VALUES
	('any');
INSERT INTO role
	(path)
VALUES
	('any');

INSERT INTO site
	(path)
VALUES
	--('any'),moved to packrat.sql
	('any.portland'),
	('any.montreal'),
	('any.playa'),
	('any.hyderabad'),
	('any.vancouver');

INSERT INTO level
	(path)
VALUES
	--('facility'), Moved ot packrat.sql
	('facility.dev01'),
	('facility.dhg'),
	('facility.cent7'),
	('facility.plasma'),
	('facility.bayou'),
	('facility.rescue'),
	('facility.mlk'),
	('facility.ssquad');

INSERT INTO platform
	(path)
VALUES
	-- ('any'), MOVED to packrat.sql
	('any.cent6_64'),
	('any.cent7_64'),
	('any.win_xp'),
	('any.win_10');

INSERT INTO role
	(path)
VALUES
	-- ('any'), MOVED TO packrat.slq
	('any.integ'),
	('any.model'),
	('any.texture'),
	('any.rig'),
	('any.anim'),
	('any.cfx'),
	('any.shotmodel'),
	('any.lookdev'),
	('any.light'),
	('any.fx'),
	('any.roto'),
	('any.enviro'),
	('any.comp'),
	('any.td'),
	('any.dpa'),
	('any.integ.beta'),
	('any.model.beta'),
	('any.texture.beta'),
	('any.rig.beta'),
	('any.anim.beta'),
	('any.cfx.beta'),
	('any.shotmodel.beta'),
	('any.lookdev.beta'),
	('any.light.beta'),
	('any.fx.beta'),
	('any.roto.beta'),
	('any.enviro.beta'),
	('any.comp.beta'),
	('any.td.beta'),
	('any.dpa.beta'),

	('any.integ.cent7'),
	('any.model.cent7'),
	('any.texture.cent7'),
	('any.rig.cent7'),
	('any.anim.cent7'),
	('any.cfx.cent7'),
	('any.shotmodel.cent7'),
	('any.lookdev.cent7'),
	('any.light.cent7'),
	('any.fx.cent7'),
	('any.roto.cent7'),
	('any.enviro.cent7'),
	('any.comp.cent7'),
	('any.td.cent7'),
	('any.dpa.cent7');

INSERT INTO package
	(name)
VALUES
	('maya'),
	('houdini'),
	('nuke'),
	('mari'),
	('vray'),
	('redshift'),
	('track'),
	('race'),
	('racetrack'),
	('snitcher'),
	('xerces'),
	('wam'),
	('wambase'),
	('modelpipeline'),
	('modelpublish'),
	('animtools'),
	('animpublish'),
	('lightpipeline'),
	('houdinipipeline'),
	('houd_out_geo'),
	('houdinicamera'),
	('nukepipeline'),
	('atomic');

INSERT INTO distribution
	(package, version)
VALUES
	
	('maya','2016.sp1'),
	('maya','2016.sp2'),
	('maya','2016.sp3'),
	('maya','2016.sp4'),
	('maya','2016.sp5'),
	('maya','2016.5.sp1'),
	('maya','2016.5.sp2'),
	('maya','2016.5.sp3'),
	('maya','2016.5.sp4'),
	('maya','2016.5.sp5'),
	('maya','2018.sp1'),
	('maya','2018.sp2'),
	('maya','2018.sp3'),
	('maya','2018.sp4'),
	('maya','2018.sp5'),
	('maya','2019.sp1'),
	('maya','2019.sp2'),
	('maya','2019.sp3'),
	('maya','2019.sp4'),
	('maya','2019.sp5'),
	('houdini','16.5.222'),
	('houdini','16.5.232'),
	('houdini','16.5.233'),
	('houdini','16.5.333'),
	('houdini','16.5.444'),
	('houdini','16.5.555'),
	('houdini','17.232'),
	('houdini','17.321'),
	('houdini','17.444'),
	('houdini','17.520'),
	('houdini','17.5.232'),
	('houdini','17.5.238'),
	('houdini','17.5.333'),
	('houdini','17.5.444'),
	('houdini','17.5.555'),
	('houdini','18.111'),
	('houdini','18.222'),
	('houdini','18.333'),
	('nuke','11.0'),
	('nuke','11.1'),
	('nuke','11.2'),
	('nuke','11.3'),
	('nuke','11.4'),
	('nuke','11.5'),
	('nuke','11.6'),
	('nuke','12.0'),
	('nuke','12.1'),
	('nuke','12.2'),
	('nuke','12.3'),
	('nuke','12.4'),
	('nuke','12.5'),
	('nuke','12.6'),
	('mari','4.0.1'),
	('mari','4.0.2'),
	('mari','4.0.3'),
	('mari','4.1.0'),
	('mari','4.1.1'),
	('mari','4.1.2'),
	('mari','4.1.3'),
	('mari','4.1.4'),
	('mari','4.1.5'),
	('mari','4.2.0'),
	('mari','4.2.1'),
	('mari','4.2.2'),
	('mari','4.2.3'),
	('mari','4.2.4'),
	('mari','4.2.5'),
	('mari','4.3.0'),
	('mari','4.3.1'),
	('mari','4.3.2'),
	('mari','4.3.3'),
	('mari','4.3.4'),
	('mari','4.3.5'),
	('mari','4.4.0'),
	('mari','4.4.1'),
	('mari','4.4.2'),
	('mari','4.4.3'),
	('mari','4.4.4'),
	('mari','4.4.5'),
	('mari','4.5.0'),
	('mari','4.5.1'),
	('mari','4.5.2'),
	('vray','3.5.0'),
	('vray','3.5.1'),
	('vray','3.5.2'),
	('vray','3.5.3'),
	('vray','3.5.4'),
	('vray','3.5.5'),
	('vray','3.6.0'),
	('vray','3.6.1'),
	('vray','3.6.2'),
	('vray','3.6.3'),
	('vray','3.6.4'),
	('vray','3.6.5'),
	('vray','3.6.6'),
	('vray','4.0.0'),
	('vray','4.0.1'),
	('vray','4.0.2'),
	('redshift','1.0.1'),
	('redshift','1.0.2'),
	('redshift','1.1.0'),
	('redshift','1.1.1'),
	('redshift','2.1.0'),
	('redshift','2.1.1'),
	('redshift','2.1.2'),
	('redshift','2.1.3'),
	('redshift','2.1.4'),
	('redshift','2.2.1'),
	('redshift','2.2.2'),
	('redshift','2.2.3'),
	('redshift','2.2.4'),
	('redshift','2.2.5'),
	('redshift','2.3.2'),
	('redshift','2.3.3'),
	('redshift','2.3.4'),
	('redshift','2.4.1'),
	('redshift','2.4.2'),
	('redshift','2.5.1'),
	('redshift','2.5.2'),
	('redshift','2.5.3'),
	('redshift','2.5.4'),
	('track','4.1.0'),
	('track','4.1.1'),
	('track','4.1.2'),
	('track','4.1.3'),
	('track','4.2.0'),
	('track','4.2.1'),
	('track','4.2.2'),
	('track','4.2.3'),
	('track','4.2.4'),
	('race','7.0.0'),
	('race','7.0.1'),
	('race','7.0.2'),
	('race','7.1.0'),
	('race','7.1.1'),
	('race','7.1.2'),
	('race','7.1.3'),
	('racetrack','7.0.0'),
	('racetrack','7.0.1'),
	('racetrack','7.0.2'),
	('racetrack','7.1.0'),
	('racetrack','7.1.1'),
	('racetrack','7.1.2'),
	('racetrack','7.1.3'),
	('snitcher','1.0.1'),
	('snitcher','1.0.2'),
	('snitcher','1.1.0'),
	('snitcher','1.1.1'),
	('snitcher','2.1.0'),
	('snitcher','2.1.1'),
	('snitcher','2.1.2'),
	('snitcher','2.1.3'),
	('snitcher','2.2.1'),
	('snitcher','2.3.2'),
	('snitcher','2.3.3'),
	('snitcher','2.3.4'),
	('snitcher','2.4.1'),
	('snitcher','2.4.2'),
	('snitcher','2.5.1'),
	('snitcher','2.5.2'),
	('snitcher','2.5.3'),
	('snitcher','2.5.4'),
	('xerces','1.0.1'),
	('xerces','1.0.2'),
	('xerces','1.1.0'),
	('xerces','1.1.1'),
	('xerces','2.1.0'),
	('xerces','2.1.1'),
	('xerces','2.1.2'),
	('xerces','2.1.3'),
	('xerces','2.2.1'),
	('xerces','2.3.2'),
	('xerces','2.3.3'),
	('xerces','2.3.4'),
	('xerces','2.3.5'),
	('xerces','2.3.6'),
	('xerces','2.3.7'),
	('xerces','2.4.1'),
	('xerces','2.4.2'),
	('xerces','2.4.3'),
	('xerces','2.4.4'),
	('xerces','2.4.5'),
	('xerces','2.5.1'),
	('xerces','2.5.2'),
	('xerces','2.5.3'),
	('xerces','2.5.4'),
	('xerces','2.6.1'),
	('xerces','2.6.2'),
	('xerces','2.6.3'),
	('xerces','2.6.4'),
	('wam','1.0.0'),
	('wam','1.0.1'),
	('wam','1.0.2'),
	('wam','1.0.3'),
	('wam','1.1.0'),
	('wam','1.1.1'),
	('wam','1.1.2'),
	('wam','1.1.3'),
	('wam','1.1.4'),
	('wam','1.1.5'),
	('wam','1.1.6'),
	('wam','1.2.0'),
	('wam','1.2.1'),
	('wam','1.2.2'),
	('wam','1.2.3'),
	('wam','1.2.4'),
	('wam','1.2.5'),
	('wam','1.2.6'),
	('wam','1.3.0'),
	('wam','1.3.1'),
	('wam','1.3.2'),
	('wam','1.3.3'),
	('wam','1.3.4'),
	('wam','1.3.5'),
	('wam','1.3.6'),
	('wambase','1.0.0'),
	('wambase','1.0.1'),
	('wambase','1.0.2'),
	('wambase','1.0.3'),
	('wambase','1.1.0'),
	('wambase','1.1.1'),
	('wambase','1.1.2'),
	('wambase','1.1.3'),
	('wambase','1.1.4'),
	('wambase','2.0.0'),
	('wambase','2.0.1'),
	('wambase','2.0.2'),
	('wambase','2.0.3'),
	('wambase','2.0.4'),
	('wambase','2.0.5'),
	('wambase','2.0.6'),
	('wambase','2.1.0'),
	('wambase','2.1.1'),
	('wambase','2.1.2'),
	('wambase','2.1.3'),
	('wambase','2.1.4'),
	('modelpublish','1.0.0'),
	('modelpublish','1.0.1'),
	('modelpublish','1.0.2'),
	('modelpublish','1.0.3'),
	('modelpublish','1.1.0'),
	('modelpublish','1.1.1'),
	('modelpublish','1.1.2'),
	('modelpublish','1.1.3'),
	('modelpublish','1.1.4'),
	('modelpublish','1.1.5'),
	('modelpublish','1.1.6'),
	('modelpublish','2.0.0'),
	('modelpublish','2.0.1'),
	('modelpublish','2.0.2'),
	('modelpublish','2.0.3'),
	('modelpipeline','1.0.0'),
	('modelpipeline','1.0.1'),
	('modelpipeline','1.0.2'),
	('modelpipeline','1.0.3'),
	('modelpipeline','1.1.0'),
	('modelpipeline','1.1.1'),
	('modelpipeline','1.1.2'),
	('modelpipeline','1.1.3'),
	('modelpipeline','1.1.4'),
	('modelpipeline','1.1.5'),
	('modelpipeline','1.1.6'),
	('modelpipeline','1.2.0'),
	('modelpipeline','1.2.1'),
	('modelpipeline','1.2.2'),
	('modelpipeline','1.2.3'),
	('modelpipeline','1.2.4'),
	('modelpipeline','1.2.5'),
	('modelpipeline','1.2.6'),
	('animtools','1.0.0'),
	('animtools','1.0.1'),
	('animtools','1.0.2'),
	('animtools','1.0.3'),
	('animtools','1.1.0'),
	('animtools','1.1.1'),
	('animtools','1.1.2'),
	('animtools','1.1.3'),
	('animtools','1.1.4'),
	('animtools','1.1.5'),
	('animtools','1.1.6'),
	('animtools','2.0.0'),
	('animtools','2.0.1'),
	('animtools','2.0.2'),
	('animtools','2.0.3'),
	('animtools','2.0.4'),
	('animtools','2.0.5'),
	('animpublish','1.0.0'),
	('animpublish','1.0.1'),
	('animpublish','1.0.2'),
	('animpublish','1.0.3'),
	('animpublish','1.1.0'),
	('animpublish','1.1.1'),
	('animpublish','1.1.2'),
	('animpublish','1.1.3'),
	('animpublish','1.1.4'),
	('animpublish','1.1.5'),
	('animpublish','1.1.6'),
	('animpublish','2.0.0'),
	('animpublish','2.0.1'),
	('animpublish','2.0.2'),
	('animpublish','2.0.3'),
	('lightpipeline','1.0.0'),
	('lightpipeline','1.0.1'),
	('lightpipeline','1.0.2'),
	('lightpipeline','1.0.3'),
	('lightpipeline','1.1.0'),
	('lightpipeline','1.1.1'),
	('lightpipeline','1.1.2'),
	('lightpipeline','1.1.3'),
	('lightpipeline','1.1.4'),
	('lightpipeline','1.1.5'),
	('lightpipeline','1.1.6'),
	('lightpipeline','2.0.0'),
	('lightpipeline','2.0.1'),
	('lightpipeline','2.0.2'),
	('lightpipeline','2.0.3'),
	('houdinipipeline','1.0.0'),
	('houdinipipeline','1.0.1'),
	('houdinipipeline','1.0.2'),
	('houdinipipeline','1.0.3'),
	('houdinipipeline','1.1.0'),
	('houdinipipeline','1.1.1'),
	('houdinipipeline','1.1.2'),
	('houdinipipeline','1.1.3'),
	('houdinipipeline','1.1.4'),
	('houdinipipeline','1.1.5'),
	('houdinipipeline','1.1.6'),
	('houdinipipeline','1.2.0'),
	('houdinipipeline','1.2.1'),
	('houdinipipeline','1.2.2'),
	('houdinipipeline','1.2.3'),
	('houdinipipeline','1.2.4'),
	('houdinipipeline','1.2.5'),
	('houdinipipeline','1.2.6'),
	('houdinipipeline','2.0.0'),
	('houdinipipeline','2.0.0.beta1'),
	('houdinipipeline','2.0.1'),
	('houdinipipeline','2.0.2'),
	('houdinipipeline','2.0.3'),
	('houdinipipeline','2.0.4'),
	('houd_out_geo','1.0.0'),
	('houd_out_geo','1.0.1'),
	('houd_out_geo','1.0.2'),
	('houd_out_geo','1.0.3'),
	('houd_out_geo','1.1.0'),
	('houd_out_geo','1.1.1'),
	('houd_out_geo','1.1.2'),
	('houd_out_geo','1.1.3'),
	('houd_out_geo','1.1.4'),
	('houd_out_geo','1.1.5'),
	('houd_out_geo','1.1.6'),
	('houd_out_geo','1.2.0'),
	('houd_out_geo','1.2.1'),
	('houd_out_geo','1.2.2'),
	('houd_out_geo','1.2.3'),
	('houd_out_geo','1.2.4'),
	('houd_out_geo','1.2.5'),
	('houd_out_geo','1.2.6'),
	('houd_out_geo','2.0.0'),
	('houd_out_geo','2.0.0.beta1'),
	('houd_out_geo','2.0.1'),
	('houd_out_geo','2.0.1.beta1'),
	('houd_out_geo','2.0.1.beta2'),
	('houd_out_geo','2.0.2'),
	('houd_out_geo','2.0.3'),
	('houd_out_geo','2.0.4'),
	('houdinicamera','1.0.0'),
	('houdinicamera','1.0.1'),
	('houdinicamera','1.0.2'),
	('houdinicamera','1.0.3'),
	('houdinicamera','1.1.0'),
	('houdinicamera','1.1.1'),
	('houdinicamera','1.1.2'),
	('houdinicamera','1.1.3'),
	('houdinicamera','1.1.4'),
	('houdinicamera','1.1.5'),
	('houdinicamera','1.1.6'),
	('houdinicamera','1.2.0'),
	('houdinicamera','1.2.1'),
	('houdinicamera','1.2.2'),
	('houdinicamera','1.2.3'),
	('houdinicamera','1.2.4'),
	('houdinicamera','1.2.5'),
	('houdinicamera','1.2.6'),
	('houdinicamera','1.3.0'),
	('houdinicamera','1.3.1'),
	('houdinicamera','1.3.2'),
	('houdinicamera','1.3.3'),
	('houdinicamera','1.3.4'),
	('houdinicamera','1.3.5'),
	('houdinicamera','1.3.6'),
	('houdinicamera','1.3.7'),
	('houdinicamera','1.3.8'),
	('houdinicamera','1.3.9'),
	('houdinicamera','1.3.10'),
	('houdinicamera','2.0.0.beta1'),
	('houdinicamera','2.0.1.beta1'),
	('houdinicamera','2.0.1.beta2'),
	('houdinicamera','2.0.2'),
	('houdinicamera','2.0.3'),
	('houdinicamera','2.0.4'),
	('houdinicamera','2.0.5'),
	('houdinicamera','2.0.6'),
	('houdinicamera','2.0.7'),
	('houdinicamera','2.0.8'),
	('houdinicamera','2.0.9'),
	('atomic','1.0.0'),
	('atomic','1.0.1'),
	('atomic','1.0.2'),
	('atomic','1.0.3'),
	('atomic','1.0.4'),
	('atomic','1.0.5'),
	('atomic','1.0.6'),
	('atomic','1.1.0'),
	('atomic','1.1.1'),
	('atomic','1.1.2'),
	('atomic','1.1.3'),
	('atomic','1.1.4'),
	('atomic','1.1.5'),
	('atomic','1.1.6'),
	('atomic','2.0.0'),
	('atomic','2.0.1'),
	('atomic','2.0.2'),
	('atomic','2.0.3'),
	('atomic','3.0.0'),
	('atomic','3.0.1'),
	('atomic','3.0.2'),
	('atomic','3.0.3'),
	('atomic','4.0.0'),
	('atomic','4.0.1'),
	('atomic','4.0.2'),
	('atomic','4.0.3'),
	('atomic','4.1.0'),
	('atomic','4.1.1'),
	('atomic','4.1.2'),
	('atomic','4.1.3'),
	('atomic','4.1.4'),
	('atomic','4.1.5'),
	('atomic','4.1.6'),
	('atomic','5.0.0'),
	('atomic','5.0.1'),
	('atomic','5.0.2'),
	('atomic','5.0.3'),
	('atomic','5.1.0'),
	('atomic','5.1.1'),
	('atomic','5.1.2'),
	('atomic','5.1.3'),
	('atomic','5.1.4'),
	('atomic','5.1.5'),
	('atomic','5.1.6'),
	('atomic','6.0.0'),
	('atomic','6.0.1'),
	('atomic','6.0.2'),
	('atomic','6.0.3'),
	('atomic','6.1.0'),
	('atomic','6.1.1'),
	('atomic','6.1.2'),
	('atomic','6.1.3'),
	('atomic','6.1.4'),
	('atomic','6.1.5'),
	('atomic','6.1.6');


INSERT INTO pincoord
	(role, level, site, platform, package)
VALUES
	-- FACILITY
	--   maya
	('any', 'facility', 'any', 'any', 'maya'),-- 2016.sp1
	('any', 'facility', 'any.hyderabad', 'any', 'maya'), -- 2018.sp3
	('any.td.beta', 'facility', 'any', 'any', 'maya'), --'maya.2019.sp2'
	--   houdini
	('any', 'facility', 'any', 'any', 'houdini'), -- 'houdini.16.5.232'
	('any.fx', 'facility', 'any', 'any', 'houdini' ), -- 'houdini.16.5.233'
	('any', 'facility', 'any.portland', 'any', 'houdini'), --'houdini.16.5.333'
	--   nuke
	('any', 'facility', 'any', 'any', 'nuke'), --'nuke.11.0'
	('any.comp', 'facility', 'any', 'any', 'nuke'), --'nuke.11.1'
	('any.comp.beta', 'facility', 'any', 'any', 'nuke' ), --'nuke.11.2'
	-- mari
	('any', 'facility', 'any', 'any', 'mari'), --'mari.4.4.0' 
	-- track
	('any', 'facility', 'any', 'any', 'track'), -- 'track.4.2.0'
	-- race
	('any', 'facility', 'any', 'any', 'race'),--'race.7.0.0'
	('any', 'facility', 'any.hyderabad', 'any', 'race'), --'race.7.0.2'
	-- racetrack
	('any', 'facility', 'any', 'any', 'racetrack' ),--'racetrack.7.0.0'
	('any', 'facility', 'any.hyderabad', 'any', 'racetrack'), --'racetrack.7.0.2'
	--snitcher
	('any', 'facility', 'any', 'any', 'snitcher'), --'snitcher.2.1.0'
	-- xerces
	('any', 'facility', 'any', 'any', 'xerces'),--'xerces.2.1.3'
	-- wam
	('any', 'facility', 'any', 'any', 'wam'), --'wam.1.1.4'
	-- wambase
	('any', 'facility', 'any', 'any', 'wambase'), -- 'wambase.2.0.4'
	-- modelpublish
	('any', 'facility', 'any', 'any', 'modelpublish' ),--'modelpublish.1.1.3'
	-- modelpipeline
	('any', 'facility', 'any', 'any', 'modelpipeline' ),--'modelpipeline.1.1.4'
	-- animtools
	('any', 'facility', 'any', 'any', 'animtools' ), --'animtools.2.0.0'
	-- animpublish
	('any', 'facility', 'any', 'any', 'animpublish' ),--'animpublish.1.1.2'
	-- lightpipeline
	('any', 'facility', 'any', 'any', 'lightpipeline' ), --'lightpipeline.1.1.6'
	-- houdinipipeline
	('any', 'facility', 'any', 'any', 'houdinipipeline' ),--'houdinipipeline.1.1.4'
	-- houd_out_geo
	('any', 'facility', 'any', 'any', 'houd_out_geo'),--'houd_out_geo.1.1.4'
	-- houdinicamera
	('any', 'facility', 'any', 'any', 'houdinicamera' ), --'houdinicamera.1.1.3'
	-- atomic
	('any', 'facility', 'any', 'any', 'atomic' ), --'atomic.5.1.4'
	--  vray
	('any', 'facility', 'any', 'any', 'vray' ),--'vray.3.6.2'
	('any.light', 'facility', 'any', 'any', 'vray' ),
	('any.light.beta', 'facility', 'any', 'any', 'vray' ),
	('any.model', 'facility', 'any', 'any', 'vray' ),--'vray.3.6.3'
	('any.model.beta', 'facility', 'any', 'any', 'vray' ),--'vray.3.6.4'
	--    redshift
	('any', 'facility', 'any', 'any', 'redshift' ), --'redshift.2.1.0'
	('any.light', 'facility', 'any', 'any', 'redshift' ), -- 'redshift.2.1.1'
	('any.light.beta', 'facility', 'any', 'any', 'redshift'), -- 'redshift.2.1.2'
	('any.model', 'facility', 'any', 'any', 'redshift' ),
	('any.model.beta', 'facility', 'any', 'any', 'redshift'), --'redshift.2.1.2'

	-- BAYOU
	-- nuke
	('any', 'facility.bayou', 'any', 'any', 'nuke'), --'nuke.11.5'
	('any.comp', 'facility.bayou', 'any', 'any', 'nuke' ), --'nuke.12.1'
	('any.comp.cent7', 'facility.bayou', 'any', 'any.cent7_64', 'nuke' ), --'nuke.12.2'
	--   maya
	('any', 'facility.bayou', 'any', 'any', 'maya' ), --'maya.2018.sp2'
	('any', 'facility.bayou', 'any', 'any.cent7_64', 'maya' ), --'maya.2018.sp5'
	('any.model', 'facility.bayou', 'any', 'any', 'maya' ),--'maya.2018.sp5'
	('any.model.beta', 'facility.bayou', 'any', 'any', 'maya'), --'maya.2019.sp4'
	-- houdini
	('any', 'facility.bayou', 'any', 'any', 'houdini'),--'houdini.16.5.444'
	('any.fx', 'facility.bayou', 'any', 'any', 'houdini' ), --'houdini.16.5.555'
	('any.fx.beta', 'facility.bayou', 'any', 'any.cent7_64', 'houdini' ), --'houdini.17.5.232'
	-- houdinipipeline
	('any.fx.beta', 'facility.bayou', 'any', 'any', 'houdinipipeline' ),--'houdinipipeline.2.0.0.beta1'
	-- houd_out_geo
	('any.fx.beta', 'facility.bayou', 'any', 'any', 'houd_out_geo' ),--'houd_out_geo.2.0.1.beta1'
	-- redshift
	('any', 'facility.bayou', 'any', 'any', 'redshift'), --'redshift.2.2.3'
	-- vray
	('any', 'facility.bayou', 'any', 'any', 'vray' ), --'vray.3.6.3'
	('any.light.beta', 'facility.bayou', 'any', 'any','vray'), -- 'vray.3.6.6'

	-- PLASMA
	--  nuke
	('any', 'facility.plasma', 'any', 'any', 'nuke' ), --'nuke.11.6'
	('any.comp', 'facility.plasma', 'any', 'any', 'nuke'), --'nuke.12.0'
	('any.comp.cent7', 'facility.plasma', 'any', 'any.cent7_64', 'nuke' ), --'nuke.12.1'
	--   maya
	('any', 'facility.plasma', 'any', 'any', 'maya' ),--'maya.2018.sp2'
	('any', 'facility.plasma', 'any', 'any.cent7_64', 'maya' ), --'maya.2018.sp5'
	('any.model', 'facility.plasma', 'any', 'any', 'maya' ), --'maya.2018.sp3'
	('any.model.beta', 'facility.plasma', 'any', 'any', 'maya' ),--'maya.2019.sp4'
	-- houdini
	('any', 'facility.plasma', 'any', 'any', 'houdini' ), --'houdini.16.5.222'
	('any.fx', 'facility.plasma', 'any', 'any', 'houdini' ), --'houdini.16.5.555'
	('any.fx.beta', 'facility.plasma', 'any', 'any.cent7_64', 'houdini' ),--'houdini.18.111'
	-- vray
	('any', 'facility.plasma', 'any', 'any', 'vray' ),--'vray.3.6.4'
	('any.light.beta', 'facility.plasma', 'any', 'any', 'vray' ), --'vray.4.0.2'
	-- RESCUE
	-- nuke
	('any', 'facility.rescue', 'any', 'any', 'nuke' ),--'nuke.11.1'
	('any.comp', 'facility.rescue', 'any', 'any', 'nuke'), -- 'nuke.12.2'
	('any.comp.cent7', 'facility.rescue', 'any', 'any', 'nuke'	 ), --'nuke.12.1'
	-- maya
	('any', 'facility.rescue', 'any', 'any', 'maya' ), --'maya.2018.sp3'
	('any', 'facility.rescue', 'any', 'any.cent7_64', 'maya' ), --'maya.2019.sp2'
	('any.model', 'facility.rescue', 'any', 'any', 'maya' ), --'maya.2018.sp3'
	('any.model.beta', 'facility.rescue', 'any', 'any', 'maya'), -- 'maya.2019.sp3'
	-- houdini
	('any', 'facility.rescue', 'any', 'any', 'houdini' ), --'houdini.16.5.333'
	-- vray
	('any', 'facility.rescue', 'any', 'any', 'vray'), --'vray.3.6.5'
	('any.light.beta', 'facility.rescue', 'any', 'any', 'vray' ); --'vray.4.0.0'


INSERT INTO versionpin
	(coord, distribution)
VALUES
	-- FACILITY
	--   maya
	(1, 1),-- 2016.sp1
	(2, 13), -- 2018.sp3
	(3, 17), --'maya.2019.sp2'
	--   houdini
	(4,22), -- 'houdini.16.5.232'
	(5,23 ), -- 'houdini.16.5.233'
	(6, 24), --'houdini.16.5.333'
	--   nuke
	(7, 39), --'nuke.11.0'
	(8, 40), --'nuke.11.1'
	(9,41 ), --'nuke.11.2'
	-- mari
	(10, 74), --'mari.4.4.0' 
	-- track
	(11, 126), -- 'track.4.2.0'
	-- race
	(12,131 ),--'race.7.0.0'
	(13, 133), --'race.7.0.2'
	-- racetrack
	(14, 138 ),--'racetrack.7.0.0'
	(15,140 ), --'racetrack.7.0.2'
	--snitcher
	(16,149 ), --'snitcher.2.1.0'
	-- xerces
	(17,170 ),--'xerces.2.1.3'
	-- wam
	(18,199 ), --'wam.1.1.4'
	-- wambase
	(19,229), -- 'wambase.2.0.4'
	-- modelpublish
	(20,244 ),--'modelpublish.1.1.3'
	-- modelpipeline
	(21,260 ),--'modelpipeline.1.1.4'
	-- animtools
	(22, 281 ), --'animtools.2.0.0'
	-- animpublish
	(23,293 ),--'animpublish.1.1.2'
	-- lightpipeline
	(24, 312 ), --'lightpipeline.1.1.6'
	-- houdinipipeline
	(25,327 ),--'houdinipipeline.1.1.4'
	-- houd_out_geo
	(26, 349),--'houd_out_geo.1.1.4'
	-- houdinicamera
	(27,374 ), --'houdinicamera.1.1.3'
	-- atomic
	(28,448 ), --'atomic.5.1.4'
	--  vray
	(29, 91 ),--'vray.3.6.2'
	(30, 92 ),
	(31,93 ),
	(32, 92 ),--'vray.3.6.3'
	(33,93 ),--'vray.3.6.4'
	--    redshift
	(34,103 ), --'redshift.2.1.0'
	(35,104 ), -- 'redshift.2.1.1'
	(36, 105), -- 'redshift.2.1.2'
	(37,104 ),
	(38, 107), --'redshift.2.1.2'

	-- BAYOU
	-- nuke
	(39, 44), --'nuke.11.5'
	(40, 47 ), --'nuke.12.1'
	(41,48 ), --'nuke.12.2'
	--   maya
	(42,12 ), --'maya.2018.sp2'
	(43,15 ), --'maya.2018.sp5'
	(44,13 ),--'maya.2018.sp5'
	(45, 19), --'maya.2019.sp4'
	-- houdini
	(46, 25),--'houdini.16.5.444'
	(47,26 ), --'houdini.16.5.555'
	(48, 27 ), --'houdini.17.5.232'
	-- houdinipipeline
	(49,336 ),--'houdinipipeline.2.0.0.beta1'
	-- houd_out_geo
	(50,362 ),--'houd_out_geo.2.0.1.beta1'
	-- redshift
	(51, 110), --'redshift.2.2.3'
	-- vray
	(52,92 ), --'vray.3.6.3'
	(53, 95), -- 'vray.3.6.6'

	-- PLASMA
	--  nuke
	(54,45 ), --'nuke.11.6'
	(55, 46), --'nuke.12.0'
	(56,47 ), --'nuke.12.1'
	--   maya
	(57, 12 ),--'maya.2018.sp2'
	(58,15 ), --'maya.2018.sp5'
	(59, 13 ), --'maya.2018.sp3'
	(60,19 ),--'maya.2019.sp4'
	-- houdini
	(61,21 ), --'houdini.16.5.222'
	(62,26 ), --'houdini.16.5.555'
	(63,36 ),--'houdini.18.111'
	-- vray
	(64, 93 ),--'vray.3.6.4'
	(65, 98 ), --'vray.4.0.2'
	-- RESCUE
	-- nuke
	(66, 40 ),--'nuke.11.1'
	(67, 48), -- 'nuke.12.2'
	(68,47 ), --'nuke.12.1'
	-- maya
	(69,13 ), --'maya.2018.sp3'
	(70,17 ), --'maya.2019.sp2'
	(71, 13 ), --'maya.2018.sp3'
	(72,18), -- 'maya.2019.sp3'
	-- houdini
	(73, 24 ), --'houdini.16.5.333'
	-- vray
	(74,94 ), --'vray.3.6.5'
	(75, 96 ); --'vray.4.0.0'


	------------------
	-- WITHPACKAGES --
	------------------

	INSERT INTO withpackage 
	-- id            varchar     integer
	(versionpin, package, pinorder) 
	VALUES 
-- maya@facility
	(1, 'race', 1),
	(1, 'racetrack', 2),
	(1, 'snitcher', 3),
	(1, 'xerces', 4),
	(1, 'wam', 5),
	(1, 'wambase', 6),
-- maya@facility hyderabad
	(2, 'race', 1),
	(2, 'racetrack', 2),
	(2, 'snitcher', 3),
	(2, 'xerces', 4),
	(2, 'wam', 5),
	(2, 'wambase', 6),
-- maya@facility td.beta
	(3, 'race', 1),
	(3, 'racetrack', 2),
	(3, 'snitcher', 3),
	(3, 'xerces', 4),
	(3, 'wam', 5),
	(3, 'wambase', 6),
-- houdini@facility
    (4, 'race', 1),
	(4, 'racetrack', 2),
	(4, 'snitcher', 3),
	(4, 'xerces', 4),
	(4, 'wam', 5),
	(4, 'wambase', 6),
	(4, 'houdinipipeline', 7),
	(4, 'houd_out_geo', 8),
-- houdini@facility fx role
    (5, 'race', 1),
	(5, 'racetrack', 2),
	(5, 'snitcher', 3),
	(5, 'xerces', 4),
	(5, 'wam', 5),
	(5, 'wambase', 6),
	(5, 'houdinipipeline', 7),
	(5, 'houd_out_geo', 8),
-- houdini@facility portland
    (6, 'race', 1),
	(6, 'racetrack', 2),
	(6, 'snitcher', 3),
	(6, 'xerces', 4),
	(6, 'wam', 5),
	(6, 'wambase', 6),
	(6, 'houdinipipeline', 7),
	(6, 'houd_out_geo', 8),
-- maya@bayou
	(42, 'race', 1),
	(42, 'racetrack', 2),
	(42, 'snitcher', 3),
	(42, 'xerces', 4),
	(42, 'wam', 5),
	(42, 'wambase', 6),
-- maya@bayou cent7 platform
	(43, 'race', 1),
	(43, 'racetrack', 2),
	(43, 'snitcher', 3),
	(43, 'xerces', 4),
	(43, 'wam', 5),
	(43, 'wambase', 6),
-- maya@bayou model role
	(44, 'race', 1),
	(44, 'racetrack', 2),
	(44, 'snitcher', 3),
	(44, 'xerces', 4),
	(44, 'wam', 5),
	(44, 'wambase', 6),
	(44, 'modelpipeline', 7),
	(44, 'modelpublish', 8),
	(44, 'vray', 9),
	(44, 'atomic', 10),
-- maya@bayou model.beta role
	(45, 'race', 1),
	(45, 'racetrack', 2),
	(45, 'snitcher', 3),
	(45, 'xerces', 4),
	(45, 'wam', 5),
	(45, 'wambase', 6),
	(45, 'modelpipeline', 7),
	(45, 'modelpublish', 8),
	(45, 'vray', 9),
	(45, 'atomic', 10),
-- houdini@bayou
    (46, 'race', 1),
	(46, 'racetrack', 2),
	(46, 'snitcher', 3),
	(46, 'xerces', 4),
	(46, 'wam', 5),
	(46, 'wambase', 6),
	(46, 'houdinipipeline', 7),
	(46, 'houd_out_geo', 8),
-- houdini@bayou fx
    (47, 'race', 1),
	(47, 'racetrack', 2),
	(47, 'snitcher', 3),
	(47, 'xerces', 4),
	(47, 'wam', 5),
	(47, 'wambase', 6),
	(47, 'houdinipipeline', 7),
	(47, 'houd_out_geo', 8),
	(47, 'houdinicamera', 9),
-- houdini@bayou fx.beta
    (48, 'race', 1),
	(48, 'racetrack', 2),
	(48, 'snitcher', 3),
	(48, 'xerces', 4),
	(48, 'wam', 5),
	(48, 'houdinicamera', 6),
	(48, 'wambase', 7),
	(48, 'houdinipipeline', 8),
	(48, 'houd_out_geo', 9),

-- maya@plasma
	(57, 'race', 1),
	(57, 'racetrack', 2),
	(57, 'snitcher', 3),
	(57, 'xerces', 4),
	(57, 'wam', 5),
	(57, 'wambase', 6),
-- maya@plasma cent7
	(58, 'race', 1),
	(58, 'racetrack', 2),
	(58, 'snitcher', 3),
	(58, 'xerces', 4),
	(58, 'wam', 5),
	(58, 'wambase', 6),
-- maya@plasma model role
	(59, 'race', 1),
	(59, 'racetrack', 2),
	(59, 'snitcher', 3),
	(59, 'xerces', 4),
	(59, 'wam', 5),
	(59, 'wambase', 6),
	(59, 'modelpipeline', 7),
	(59, 'modelpublish', 8),
	(59, 'vray', 9),
	(59, 'atomic', 10),
-- maya@plasma model.beta role
	(60, 'race', 1),
	(60, 'racetrack', 2),
	(60, 'snitcher', 3),
	(60, 'xerces', 4),
	(60, 'wam', 5),
	(60, 'wambase', 6),
	(60, 'modelpipeline', 7),
	(60, 'modelpublish', 8),
	(60, 'vray', 9),
	(60, 'atomic', 10),
-- houdini@plasma
    (61, 'race', 1),
	(61, 'racetrack', 2),
	(61, 'snitcher', 3),
	(61, 'xerces', 4),
	(61, 'wam', 5),
	(61, 'wambase', 6),
	(61, 'houdinipipeline', 7),
	(61, 'houd_out_geo', 8),
-- houdini@plasma
    (62, 'race', 1),
	(62, 'racetrack', 2),
	(62, 'snitcher', 3),
	(62, 'xerces', 4),
	(62, 'wam', 5),
	(62, 'wambase', 6),
	(62, 'houdinipipeline', 7),
	(62, 'houd_out_geo', 8),
-- houdini@plasma
    (63, 'race', 1),
	(63, 'racetrack', 2),
	(63, 'snitcher', 3),
	(63, 'xerces', 4),
	(63, 'wam', 5),
	(63, 'wambase', 6),
	(63, 'houdinipipeline', 7),
	(63, 'houd_out_geo', 8),
-- maya@rescue
	(69, 'race', 1),
	(69, 'racetrack', 2),
	(69, 'snitcher', 3),
	(69, 'xerces', 4),
	(69, 'wam', 5),
	(69, 'wambase', 6),
-- maya@rescue cent7
	(70, 'race', 1),
	(70, 'racetrack', 2),
	(70, 'snitcher', 3),
	(70, 'xerces', 4),
	(70, 'wam', 5),
	(70, 'wambase', 6),
-- maya@rescue model role
	(71, 'race',          1),
	(71, 'racetrack',     2),
	(71, 'snitcher',      3),
	(71, 'xerces',        4),
	(71, 'wam',           5),
	(71, 'wambase',       6),
	(71, 'modelpipeline', 7),
	(71, 'modelpublish',  8),
	(71, 'vray',          9),
	(71, 'atomic',        10),
-- maya@rescue model.beta role
	(72, 'race',          10),
	(72, 'racetrack',     2),
	(72, 'snitcher',      3),
	(72, 'xerces',        4),
	(72, 'wam',           5),
	(72, 'wambase',       6),
	(72, 'modelpipeline', 7),
	(72, 'modelpublish',  8),
	(72, 'vray',          9),
	(72, 'atomic',        1),
-- houdini@rescue
    (73, 'race', 1),
	(73, 'racetrack', 2),
	(73, 'snitcher', 3),
	(73, 'xerces', 4),
	(73, 'wam', 5),
	(73, 'wambase', 6),
	(73, 'houdinipipeline', 7),
	(73, 'houd_out_geo', 8);