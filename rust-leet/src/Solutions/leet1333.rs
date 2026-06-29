use macros::sol_macro;
sol_macro!();

#[derive(Debug)]
struct restau {
    id: i32,
    rating: i32,
    veganF:bool,
    price: i32,
    distance:i32,
}

impl Solution {
    pub fn filter_restaurants(restaurants: Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32> {
        let mut rests: Vec<restau> = Vec::new();
        for x in restaurants{
            let new_res = restau{
                id: x[0],
                rating: x[1],
                veganF: x[2] == 1,
                price: x[3],
                distance: x[4],
            };
            rests.push(new_res);
        }
        rests = rests.into_iter().filter(|res| {
            (!(vegan_friendly == 1) || res.veganF ) && res.price <= max_price && res.distance <= max_distance
        }).collect();

        rests.sort_by(|a, b| {
            b.rating.cmp(&a.rating).then_with(|| b.id.cmp(&a.id))
        });
        println!("{:?}",rests);
        let res:Vec<i32> = rests.into_iter().map(|r| r.id).collect();
        return res;
    }
}