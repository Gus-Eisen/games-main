mod game_collection;

use pelican_ui::{Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS, HardwareContext, runtime::Services};
use pelican_ui::drawable::Drawable;
use pelican_ui_std::{AvatarIconStyle, AvatarContent, Interface, NavigateEvent, AppPage};
use pelican_ui::runtime::{Service, ServiceList};
use std::any::TypeId;
use std::pin::Pin;
use std::future::Future;
use pelican_ui::events::{Event, Key, KeyboardEvent, KeyboardState, NamedKey};
use std::collections::BTreeMap;
use std::os::unix::raw::mode_t;
use std::ptr::addr_of_mut;
use image::{load_from_memory, RgbaImage};
use pelican_ui::include_assets;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
//TODO: delete commented out code if lib.rs is not required for colored background.
//use pelican_ui::drawable::Color;

use crate::game_collection::airstrike_game::airstrike::Airstrike;
use crate::game_collection::airstrike_game::player::Player;
use crate::game_collection::airstrike_game::server::{ArduinoServer, GameAction};

pub struct MyApp;

impl Services for MyApp {
    fn services() -> ServiceList {
        ServiceList::default()
    }
}

impl Plugins for MyApp {
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
        vec![]
    }
}

impl Application for MyApp {
    //looks like where I declare all sprites that will be used.
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        //which folder I can load assets from.
        ctx.assets.include_assets(include_assets!("./assets"));
        let mut illustrations = ctx.theme.brand.illustrations.clone();
        illustrations.insert(ctx, "spaceship", "spaceship.png");
        illustrations.insert(ctx, "b2", "b2.png");
        illustrations.insert(ctx, "tiki_fly", "tiki_fly.png");
        illustrations.insert(ctx, "northrop", "northrop.png");
        illustrations.insert(ctx, "bullet_downward", "bullet_downward.png");
        illustrations.insert(ctx, "bullet_blue", "bullet_blue.png");
        illustrations.insert(ctx, "explosion", "explosion.png");
        illustrations.insert(ctx, "f117", "f117.png");
        illustrations.insert(ctx, "player_lives", "player_lives.png");
        ctx.theme.brand.illustrations = illustrations;

        let game = Games::Airstrike.init(ctx);
        Box::new(Interface::new(ctx, game, None))
    }
}

start!(MyApp);

enum Games {
    Airstrike
}

impl Games {
    pub fn init(&self, ctx: &mut Context) -> Box<dyn AppPage> {
        match self {
            Games::Airstrike => Box::new(Airstrike::new(ctx, None))
        }
    }
}