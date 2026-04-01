pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    // so the question is asking, when will the lower position cars reach higher position cars
    // do I start from the lowest position car? might not be because there will be other position
    // cars reach other position cars
    // can I calculate when will each car reach destination?
    // for those cars whose position <= other cars, and time reaching destination <= other cars
    // beccomes fleet
    // I can then calculate time at dest for each car
    // one thing to note is, once become a fleet, the faster car automatically becomes slower
    // start from the lowest position
    // if its time at dest <= future cars, they are one fleet
    // for when iterating time at dest from the lowest position car
    // when seeing a time is larger or equal, same fleet
    // when seeing a time is smaller, different fleet
    // keep a variable of curr fleet dest time

    // mistake: didn't think of a case
    // where a later fleet actually becomes very slow

    // to tackle this, I need to iterate from backward?
    // a later position car should arraive faster, hence smaller the dest time
    // when iterating backward, when seeing a smaller or equal time, its the same fleet
    // their dest time will be the bigger among the two
    let mut dest_time = Vec::new();
    for i in 0..position.len() {
        dest_time.push((target - position[i]) as f64 / speed[i] as f64);
    }

    let mut position_dest: Vec<(i32, f64)> = position
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, dest_time[i]))
        .collect();
    position_dest.sort_by_key(|(p, _)| *p);

    let mut res = 0;
    let mut prev_fleet_dest = f64::MIN;
    for (p, d) in position_dest.into_iter().rev() {
        // slower
        if d > prev_fleet_dest {
            res += 1;
            prev_fleet_dest = d;
        }
        // faster
        // else {
        //     prev_fleet_dest = d;
        // }
    }
    // let mut prev_fleet_dest = f64::MAX;
    // for (p, d) in position_dest {
    //     // faster
    //     if d <= prev_fleet_dest {
    //         res += 1;
    //         prev_fleet_dest = d;
    //     }
    //     // faster
    //     // else {
    //     //     prev_fleet_dest = d;
    //     // }
    // }

    res
}
