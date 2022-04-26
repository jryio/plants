-- Add up migration script here

-- Install UUID extension for postgres
create extension if not exists "uuid-ossp";

set timezone = 'America/New_York';

-- Create location table
create table if not exists locations (
  id uuid default uuid_generate_v4(),
  name varchar(50),
  primary key (id)
);

-- Create people table
create table if not exists people (
  id uuid default uuid_generate_v4(),
  name varchar(100),
  primary key (id)
);

-- Create plants table
create table if not exists plants (
  id uuid default uuid_generate_v4(),
  name varchar(100) not null,
  location uuid not null,
  birthday timestamptz default NOW(),
  image varchar not null,
  watering_frequency smallint check(watering_frequency > 0),
  watering_instructions varchar not null,
  last_watered_date timestamptz not null,
  last_watered_by uuid,
  -- keys
  primary key (id),
  constraint fk_location foreign key (location) references locations(id),
  constraint fk_person foreign key (last_watered_by) references people(id)
);
