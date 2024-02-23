-- categories summary
select id,
    enum_id,
    value,
    tots.entries
from categories
    join (
        select parent_id,
            count(*) as entries
        from categories
        where parent_id is not null
        group by parent_id
    ) as tots on categories.id = tots.parent_id
where categories.parent_id is null
order by categories.id;


select parent_id,
    count(*)
from categories
where parent_id is not null
group by parent_id;