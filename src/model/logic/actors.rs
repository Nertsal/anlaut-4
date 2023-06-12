use super::*;

impl Model {
    pub(super) fn actors_ai(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a Position,
            #[query(storage = ".body")]
            velocity: &'a mut vec2<Coord>,
            stats: &'a Stats,
            controller: &'a mut Controller,
            #[query(optic = "._Some")]
            ai: &'a mut ActorAI,
            gun: &'a mut Option<Gun>,
            stunned: &'a Option<Time>,
        }

        let player = self
            .actors
            .get(self.player.actor)
            .expect("Player actor not found")
            .clone();

        let mut shots = Vec::new();

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_, actor)) = iter.next() {
            if actor.stunned.is_some() {
                continue;
            }

            let player_dir = actor
                .position
                .direction(player.body.collider.position, self.config.world_size);
            // let player_dist = player_dir.len();
            let player_dir = player_dir.normalize_or_zero();

            match actor.ai {
                ActorAI::Crawler => {
                    actor.controller.target_velocity = player_dir * actor.stats.move_speed;
                }
                ActorAI::Ranger { preferred_distance } => {
                    let target = player
                        .body
                        .collider
                        .position
                        .shifted(-player_dir * *preferred_distance, self.config.world_size);
                    let target_dir = actor
                        .position
                        .direction(target, self.config.world_size)
                        .normalize_or_zero();
                    actor.controller.target_velocity = target_dir * actor.stats.move_speed;

                    if let Some(gun) = actor.gun {
                        if gun.shot_delay <= Time::ZERO {
                            gun.shot_delay = gun.config.shot_delay;
                            let target_pos = player.body.collider.position;
                            let dir = actor.position.direction(target_pos, self.config.world_size);
                            *actor.velocity -= dir.normalize_or_zero() * gun.config.recoil;
                            shots.push((
                                *actor.position,
                                target_pos,
                                Fraction::Enemy,
                                gun.config.shot.clone(),
                            ));
                        }
                    }
                }
            }
        }

        for (pos, aimed_towards, fraction, config) in shots {
            self.shoot(pos, aimed_towards, fraction, config);
        }
    }

    pub(super) fn control_actors(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(storage = ".body")]
            velocity: &'a mut vec2<Coord>,
            controller: &'a Controller,
            stunned: &'a mut Option<Time>,
        }

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_, actor)) = iter.next() {
            let target_velocity = if let Some(time) = actor.stunned {
                *time -= delta_time;
                if *time <= Time::ZERO {
                    *actor.stunned = None;
                }
                vec2::ZERO
            } else {
                actor.controller.target_velocity
            };

            // Interpolate body velocity to target velocity.
            // Take min(1.0) to not overshoot
            *actor.velocity += (target_velocity - *actor.velocity)
                * (actor.controller.acceleration * delta_time).min(Coord::ONE);
        }
    }
}
