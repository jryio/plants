-- Add down migration script here
set timezone = 'UTC';

-- Drop order matters
-- n. table A -> (references) table B, C, D, ...
--
-- 1. watering_events -> plants, people
-- 2. plants -> people, locations
-- 3. people -> ...
-- 4. locations -> ...
drop table if exists watering_events;

drop table if exists plants;

drop table if exists locations;

drop table if exists people;

-- Then extension is no longer used by any table, so can be dropped
drop extension if exists "uuid-ossp";
