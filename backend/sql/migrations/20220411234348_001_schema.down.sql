-- Add down migration script here
set timezone = 'UTC';

-- Drop order matters. First plants table depends on people and locations
drop table if exists plants;

drop table if exists locations;

drop table if exists people;

-- Then extension is no longer used by any table, so can be dropped
drop extension if exists "uuid-ossp";
