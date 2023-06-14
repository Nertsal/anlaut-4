use super::*;

impl Model {
    pub(super) fn generate_level(&mut self) {
        let config = &self.level;
        let palette = self.theme.get_palette(&self.theme.level.foreground);
        self.blocks = generate_blocks(&config.foreground, &palette, self.config.world_size);

        let palette = self.theme.get_palette(&self.theme.level.background);
        self.background_blocks =
            generate_blocks(&config.background, &palette, self.config.world_size);
    }
}

fn generate_blocks(
    config: &ProcGenConfig,
    palette: &[Color],
    world_size: vec2<Coord>,
) -> StructOf<Arena<Block>> {
    let mut rng = thread_rng();
    let mut spawns: Vec<Position> = Vec::new();
    let mut result: StructOf<Arena<Block>> = StructOf::new();

    let max_iter = config.blocks_number * 3; // ~3 attempts per block
    for _ in 0..max_iter {
        if spawns.len() >= config.blocks_number {
            break;
        }

        let position = Position::random(&mut rng, world_size);
        if spawns
            .as_slice()
            .iter()
            .any(|pos| pos.distance(position, world_size) < config.spacing)
        {
            // Too close to another block
            continue;
        }

        let block = config
            .blocks
            .choose_weighted(&mut rng, |config| config.weight.as_f32())
            .expect("no block variants found to generate")
            .clone();

        let (color, rotation) = match block.kind {
            BlockKind::Obstacle => (
                *palette.choose(&mut rng).expect("no colors in the pallete"),
                Angle::from_degrees(rng.gen_range(0.0..360.0).as_r32()),
            ),
            BlockKind::Barrel => (Color::WHITE, Angle::ZERO),
        };

        spawns.push(position);
        result.insert(Block {
            color,
            health: block.health.map(Health::new),
            on_fire: None,
            vulnerability: block.vulnerability,
            kind: block.kind,
            collider: {
                Collider {
                    position,
                    rotation,
                    shape: block.shape,
                }
            },
            explosion: block.explosion,
        });
    }

    result
}