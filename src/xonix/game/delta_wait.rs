use web_time::SystemTime;

pub fn delta_wait(last_time: &mut f64) {
    let mut now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("A temporal paradox occurred!")
        .as_secs_f64();

    while now - *last_time < 0.017 { // 0.017
        now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs_f64();
    } 
    *last_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs_f64();
}
