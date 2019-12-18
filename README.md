# Packybara

## Notes

### versionpin notation
I need a way to set versionpins from the command line. What about this?
```bash
<distribution>@role:<role>.level:<level>.platform:<platform>.site:<site>
```
where role, level, paltform, and site pairs are optional, and their order is not fixed. 
We can also use single letter abreviations. 

```
maya-2018.sp3@l:bayou => maya-2018.sp3@level:bayou.role:any.platform:any.site:any
```


# db 
```
WITH tid AS
( 
    SELECT DISTINCT 
        transaction_id 
    FROM 
        audit.logged_actions
    WHERE
        table_name = 'package' 
    ORDER BY 
        transaction_id desc 
    LIMIT 
        1
)  
SELECT 
    table_name,
    client_addr,
    action,
    TO_JSON(row_data) AS row_data 
FROM 
    audit.logged_actions 
INNER JOIN 
    tid 
USING (transaction_id);
```

# lets agg it
```
WITH tid AS
( 
    SELECT DISTINCT 
        transaction_id 
    FROM 
        audit.logged_actions
    WHERE
        table_name = 'package' 
    ORDER BY 
        transaction_id desc 
    LIMIT 
        1
)  
SELECT
    table_name,
    action,
    '{"table": "'|| table_name ||'","' || action || '": ' || to_json(ARRAY_AGG(row_data)) || '}' AS json
FROM
    audit.logged_actions
INNER JOIN
    tid
USING (transaction_id)
GROUP BY
    action,table_name;
```

```
WITH tid AS
( 
    SELECT DISTINCT 
        transaction_id 
    FROM 
        audit.logged_actions
    WHERE
        table_name = 'package' 
    ORDER BY 
        transaction_id desc 
    LIMIT 
        1
)  
SELECT
    '{ "transaction_id: ' || transaction_id  ||', "target": "'|| table_name ||'","op": "' || LOWER(action) || '", "values": ' || to_json(ARRAY_AGG(row_data)) || '}' AS revision
FROM
    audit.logged_actions
INNER JOIN
    tid
USING (transaction_id)
GROUP BY
    transaction_id,action,table_name;
```

## Edits
### Transaction
```
 select transaction_id,table_name,action,row_data,changed_fields from audit.logged_actions where transaction_id = (select distinct transaction_id from audit.logged_actions order by transaction_id desc limit 1 offset 0);
 ```

 ```
 insert platform(path) values ('any.cent8_128'),('any.cent9_128');
 update platform set path='any.cent8_64' where path='any.cent8_128';
 ```

 ## Pulling what i need
 
```
with auditvals as 
    (
        select 
            row_data->'id' as id,
            row_data->'coord' as coord, 
            row_data->'distribution' as old_dist,
            changed_fields->'distribution' as new_dist,
            transaction_id
        from 
            audit.logged_actions
        where 
            table_name='versionpin' and 
            row_data is not null and
            action = 'UPDATE' and
        transaction_id = 13351
    ) 
select 
    auditvals.id as auditvals_id,
    pkgcoord.role_name,
    pkgcoord.level_name,
    pkgcoord.site_name,
    pkgcoord.platform_name,
    distribution.name as old_distribution,
    distribution2.name as new_distribution
from 
    pkgcoord_view as pkgcoord
inner join 
    auditvals 
on 
    auditvals.coord::integer = pkgcoord.pkgcoord_id 
inner join 
    distribution_view as distribution 
on 
    auditvals.old_dist::INTEGER = distribution.distribution_id
inner join 
    distribution_view as distribution2
on auditvals.new_dist::INTEGER = distribution2.distribution_id;
```

# How to handle inserts and updates
```
WITH auditvals AS 
    (
        SELECT 
            row_data->'id' AS id,
            row_data->'coord' AS coord, 
            row_data->'distribution' AS old_dist,
            changed_fields->'distribution' AS new_dist,
            transaction_id,
            action
        FROM 
            audit.logged_actions
        WHERE 
            table_name='versionpin' and 
            row_data is not null
    ) 
SELECT 
    auditvals.id AS auditvals_id,
    auditvals.action,
    auditvals.transaction_id,
    pkgcoord.role_name,
    pkgcoord.level_name,
    pkgcoord.site_name,
    pkgcoord.platform_name,
    distribution.name AS old_distribution,
    distribution2.name AS new_distribution
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
```