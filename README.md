# Packybara


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
    ORDER BY 
        transaction_id desc 
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

```
WITH tid AS
( 
    SELECT DISTINCT 
        transaction_id 
    FROM 
        audit.logged_actions
    ORDER BY 
        transaction_id desc 
)  
SELECT
    case 
    when action = 'INSERT' THEN 
        json_build_object('transaction_id', transaction_id,'target',table_name,'op',action, 'values',ARRAY_AGG(row_data))
    when action = 'UPDATE' THEN
        json_build_object('transaction_id',transaction_id, 'target',table_name,'op', action,'values', ARRAY_AGG( json_build_object('from',to_json(row_data),'to',to_json(changed_fields))))
    WHEN action = 'DELETE' THEN 
        json_build_object('transaction_id', transaction_id,'target',table_name,'op',action, 'values',ARRAY_AGG(row_data))
    ELSE 
        json_build_object('transaction_id', transaction_id,'target',table_name,'op',action) 
    END 
    AS revision

FROM
    audit.logged_actions
INNER JOIN
    tid
USING (transaction_id)
GROUP BY
    transaction_id,action,table_name;
```

```
WITH tid AS
( 
    SELECT DISTINCT 
        transaction_id 
    FROM 
        audit.logged_actions
    ORDER BY 
        transaction_id desc 
)  
SELECT
    table_name,
    transaction_id,
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
    AS revision

FROM
    audit.logged_actions
INNER JOIN
    tid
USING (transaction_id)
GROUP BY
    transaction_id,action,table_name;
    ```
    
```
with tid as 
(
    SELECT
    transaction_id,
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
    AS actions

FROM
    audit.logged_actions
GROUP BY
    transaction_id,action,table_name
ORDER BY
    transaction_id
)
SELECT
    i.transaction_id,
    i.table_name,
   json_build_object(
       'transaction_id', tid.transaction_id, 'transaction_type', i.table_name, 'actions',tid.actions
   )
from 
(
    SELECT
        distinct
        transaction_id,
        table_name
    from audit.logged_actions
    
)
     as i

inner join
    tid
using (transaction_id);
```


```
with tid as 
(
    SELECT
    transaction_id,
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
    )
    AS actions

FROM
    audit.logged_actions
GROUP BY
    transaction_id,action,table_name
ORDER BY
    transaction_id
)
SELECT
    i.transaction_id,
   json_build_object(
       'transaction_id', tid.transaction_id,  'actions',tid.actions
   )
from 
(
    SELECT
        distinct
        transaction_id
    from audit.logged_actions
    
)
     as i

inner join
    tid
using (transaction_id);
```

```
with tid as 
(
    SELECT
    transaction_id,
    json_build_object(table_name,
    case 
    when action = 'INSERT' THEN 
        json_build_object(action, ARRAY_AGG(row_data))
    when action = 'UPDATE' THEN
        json_build_object( action, ARRAY_AGG( json_build_object('from',to_json(row_data),'to',to_json(changed_fields))))
    WHEN action = 'DELETE' THEN 
        json_build_object(action, ARRAY_AGG(row_data))
    ELSE 
        json_build_object(action,
        ARRAY_AGG(table_name)
        ) 
    END 
    )
    AS actions

FROM
    audit.logged_actions
GROUP BY
    transaction_id,action,table_name
ORDER BY
    transaction_id
)
SELECT
    i.transaction_id,
   json_build_object(
       'transaction_id', tid.transaction_id,  'actions',tid.actions
   )
from 
(
    SELECT
        distinct
        transaction_id
    from audit.logged_actions
    
)
     as i

inner join
    tid
using (transaction_id);
```


```
SELECT 
    transaction_id, 
     jsonb_build_object('transaction_id',transaction_id,'actions',json_agg(actions)) AS changeset
FROM
(
    SELECT
    la.transaction_id,
    jsonb_build_object(
        table_name,
    CASE 
    WHEN action = 'INSERT' THEN 
        jsonb_build_object(
            action
            ,ARRAY_AGG(row_data)
        )
    WHEN action = 'UPDATE' THEN
        jsonb_build_object( 
            action, 
            ARRAY_AGG( 
                jsonb_build_object(
                    'from'
                    ,to_jsonb(row_data)
                    ,'to'
                    ,to_jsonb(changed_fields)
                 )
            )
        )
    WHEN action = 'DELETE' THEN 
        jsonb_build_object(
            action
            ,ARRAY_AGG(row_data)
        )
    ELSE 
        jsonb_build_object(
            action
            ,table_name
        ) 
    END 
    ) AS actions

FROM
    audit.logged_actions AS la
GROUP BY
    la.transaction_id 
    ,la.action
    ,la.table_name
ORDER BY
    la.transaction_id
) innerq
GROUP BY 
    transaction_id;

```

```
select 
    key,
    cast(value as integer)
   
from 
    each(
        (select 
            row_data 
        from 
            audit.logged_actions 
        limit 1) 
    ) As a
 where  position('id' in key)>0;