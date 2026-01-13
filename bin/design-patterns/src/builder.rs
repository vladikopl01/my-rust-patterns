#[derive(Debug)]
enum BurgerComponent {
    Bun,
    Patty,
    Tomato,
    Cheese,
}

#[derive(Debug)]
struct BurgerBuilder {
    components: Vec<BurgerComponent>,
}

impl BurgerBuilder {
    fn new() -> Self {
        BurgerBuilder {
            components: vec![BurgerComponent::Bun],
        }
    }

    pub fn add_component(mut self, component: BurgerComponent) -> Self {
        self.components.push(component);
        self
    }

    pub fn build(mut self) -> BurgerBuilder {
        self.components.push(BurgerComponent::Bun);
        self
    }
}

pub fn builder_pattern() {
    dbg!(
        BurgerBuilder::new()
            .add_component(BurgerComponent::Patty)
            .add_component(BurgerComponent::Cheese)
            .add_component(BurgerComponent::Tomato)
            .build()
    );
}
