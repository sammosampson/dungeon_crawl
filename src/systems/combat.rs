use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Health)]
pub fn combat(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect::<Vec<(Entity, Entity)>>();
        
    victims
        .iter()
        .for_each(|(message, victim)| {
            if let Ok(mut health) = ecs
                .entry_mut(*victim)
                .unwrap()
                .get_component_mut::<Health>() {
                    println!("Helath before attack {}", health.current);
                    health.current -= 1;
                    if health.current < 1 {
                        commands.remove(*victim);
                    }
                    println!("Helath after attack {}", health.current);
                }
            commands.remove(*message);
        });
            
}
