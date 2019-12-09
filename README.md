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