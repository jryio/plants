use chrono::prelude::*;
use chrono::Duration;
use rand::Rng;

use crate::components::plant_card::PlantCardProps;

const FIDDLE: &str = "https://hips.hearstapps.com/vader-prod.s3.amazonaws.com/1557177323-pilea-peperomioides-money-plant-in-the-pot-single-royalty-free-image-917778022-1557177295.jpg?crop=1.00xw:0.668xh;0,0.269xh&resize=480:*";
const IVY: &str = "https://res.cloudinary.com/patch-gardens/image/upload/c_fill,f_auto,h_840,q_auto:good,w_840/v1618417886/o9s5jk0fkxdm0zpp7edh.jpg";
const SUCC: &str = "https://images.ctfassets.net/i3tkg7dt3kro/VRRT4VVFz4YmbNolgl6DD/0cabac8a1e7b2b7d7d3e5f527e5c623f/mini-succulents-zebra-cactus.jpg";
const MONEY: &str = "https://www.plants.com/images/1566427725771_20190821-1566427728395.png";
const PARADISE: &str = "https://www.thespruce.com/thmb/LnEYb2dOBSJVw29eRXdUOJqm3xI=/2000x2000/smart/filters:no_upscale()/grow-velvet-plant-indoors-1902643-7-a148ac1c3bc642479acf95ab5a7f3a09.jpg";

pub fn make_plant_previews(n: usize) -> Vec<PlantCardProps> {
    (0..=n)
        .map(|i| {
            let name = String::from("Fiddle Leaf Fig");
            let images = vec![FIDDLE, IVY, SUCC, MONEY, PARADISE];
            let image = images[i % images.len()].to_string();
            let room = String::from("");
            let water_frequency: usize = 5;
            let water_instructions = String::from("");
            let last_watered_date = {
                let rand_num_days: i64 = rand::thread_rng().gen_range(0..=30);
                let duration = Duration::days(rand_num_days);
                Utc::now() + duration
            };
            let last_watered_by = {
                match i % 2 == 0 {
                    true => String::from("Magda"),
                    false => String::from("Jacob"),
                }
            };
            PlantCardProps {
                name,
                image,
                room,
                water_frequency,
                water_instructions,
                last_watered_date,
                last_watered_by,
            }
            // plants.push(props)
        })
        .collect::<Vec<PlantCardProps>>()
}
