mod misc;
mod wallet;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::scope("/lookup").configure(lookup::config));

    cfg.configure(wallet::config);
    cfg.configure(misc::config);
    // cfg.configure(transactions::config);
    // cfg.configure(ws::config);
    // cfg.configure(names::config);
}
