use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::db::get_db;

use super::{location::Location, person::Person, plant::Plant, watering_event::WateringEvent};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // ----------------
    // Plant
    // ----------------
    pub async fn plants(&self, ctx: &Context<'_>) -> Result<Vec<Plant>> {
        let plants = sqlx::query_as!(
            Plant,
            r#"
            select
                plants.id as "id!",
                plants.name as "name!",
                location as "location_id!",
                birthday as "birthday!",
                image as "image!",
                watering_frequency as "watering_frequency!",
                watering_instructions as "watering_instructions!",
                last_watered_date as "last_watered_date!",
                last_watered_by as "last_watered_by!"
           from plants
            "#
        )
        .fetch_all(get_db(ctx)?)
        .await?;
        Ok(plants)
    }

    /// Get exactly one plant by its UUID
    pub async fn plant(&self, ctx: &Context<'_>, plant_id: Uuid) -> Result<Plant> {
        let plant = sqlx::query_as!(
            Plant,
            r#"
            select
                id as "id!",
                name as "name!",
                location as "location_id!",
                birthday as "birthday!",
                image as "image!",
                watering_frequency as "watering_frequency!",
                watering_instructions as "watering_instructions!",
                last_watered_date as "last_watered_date!",
                last_watered_by as "last_watered_by!"
            from plants
            where
                id = $1
            "#,
            // FIXME: This input needs to be sanitized
            plant_id
        )
        .fetch_one(get_db(ctx)?)
        .await?;
        Ok(plant)
    }

    // ----------------
    // Location
    // ----------------

    /// locations() returns all of the locations where plants live. Create a new location if one does
    /// not already exist for the room the plant is in.
    pub async fn locations(&self, ctx: &Context<'_>) -> Result<Vec<Location>> {
        let locations = sqlx::query_as!(
            Location,
            r#"select id as "id!", name as "name!" from locations"#
        )
        .fetch_all(get_db(ctx)?)
        .await?;
        Ok(locations)
    }

    /// location() returns the id and name of a single loaction by its Uuid.
    pub async fn location(&self, ctx: &Context<'_>, location_id: Uuid) -> Result<Location> {
        let location = sqlx::query_as!(
            Location,
            r#"select id as "id!", name as "name!" from locations where id = $1"#,
            location_id
        )
        .fetch_one(get_db(ctx)?)
        .await?;
        Ok(location)
    }

    // ----------------
    // People
    // ----------------

    /// people() returns all of the people in the household who can water plants
    pub async fn people(&self, ctx: &Context<'_>) -> Result<Vec<Person>> {
        let people = sqlx::query_as!(Person, r#"select id as "id!", name as "name!" from people"#)
            .fetch_all(get_db(ctx)?)
            .await?;
        Ok(people)
    }

    // person() returns the id and name of a single person who can water plants
    pub async fn person(&self, ctx: &Context<'_>, person_id: Uuid) -> Result<Person> {
        let person = sqlx::query_as!(
            Person,
            r#"select id as "id!", name as "name!" from people where id = $1"#,
            person_id
        )
        .fetch_one(get_db(ctx)?)
        .await?;
        Ok(person)
    }

    // ----------------
    // Watering Event
    // ----------------

    /// watering_events() returns all of the times which a plant was watered, including which plant,
    /// which person, what time, a photograph, and notes about the plant or watering event at the
    /// time.
    pub async fn watering_events(&self, ctx: &Context<'_>) -> Result<Vec<WateringEvent>> {
        let watering_events = sqlx::query_as!(
            WateringEvent,
            r#"
            select
                id as "id!",
                plant_id as "plant_id!",
                person_id as "person_id!",
                ctime as "ctime!",
                notes as "notes!"
            from watering_events
            "#
        )
        .fetch_all(get_db(ctx)?)
        .await?;
        Ok(watering_events)
    }

    /// watering_event() returns the information for a single WateringEvent
    pub async fn watering_event(
        &self,
        ctx: &Context<'_>,
        watering_event_id: Uuid,
    ) -> Result<WateringEvent> {
        let watering_event = sqlx::query_as!(
            WateringEvent,
            r#"
            select
            id as "id!", plant_id as "plant_id!", person_id as "person_id!", ctime as "ctime!", notes as "notes!"
            from watering_events
            where
            id = $1
            "#,
            watering_event_id
        ).fetch_one(get_db(ctx)?).await?;
        Ok(watering_event)
    }
}
