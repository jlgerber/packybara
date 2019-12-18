SELECT audit.audit_table('versionpin');
SELECT audit.audit_table('revision');
SELECT audit.audit_table('package');
SELECT audit.audit_table('level');
SELECT audit.audit_table('site');
SELECT audit.audit_table('role');
SELECT audit.audit_table('platform');
SELECT audit.audit_table('distribution'); 
SELECT audit.audit_table('withpackage');


CREATE OR REPLACE VIEW revision_view AS (
    WITH cte AS 
        (
            SELECT
                transaction_id,
                row_data->'id' AS revision_id,
                action_tstamp_tx as datetime
            FROM 
                audit.logged_actions 
            WHERE 
                table_name ='revision'
        ) 
    SELECT 
        id,
        transaction_id, 
        author, 
        datetime,
        comment
    FROM 
        revision 
    JOIN 
        cte 
    ON 
        revision.id = cte.revision_id::INTEGER
);
