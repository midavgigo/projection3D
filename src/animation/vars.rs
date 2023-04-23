pub mod vars{
    use std::collections::HashMap;

    pub struct Commands{
        commands: HashMap<u8, Vec<u8>>
    }
    impl Commands{
        pub fn init()->Self{
            let commands:HashMap<u8, Vec<u8>> = HashMap::from([
                (65, vec![1, 4]),//A
                (66, vec![1, 4]),//B
                (67, vec![1, 4, 1, 1]),//C
                (74, vec![1]),//J
                (77, vec![1, 1, 4, 4, 4]),//M
                (81, vec![1, 4]),//P
                (82, vec![1, 1, 4, 4, 4, 4]),//R
                (83, vec![1, 1, 4, 4, 4]),//S
                (84, vec![1, 1, 4, 1, 1]),//T
                (87, vec![1, 1]),//W
                (97, vec![1, 1]),//a
                (98, vec![1, 1]),//b
                (99, vec![1, 1, 1, 1]),//c
                (106, vec![1]),//j
                (109, vec![1, 1, 1, 1, 1]),//m
                (113, vec![1, 1]),//p
                (114, vec![1, 1, 1, 1, 1, 1]),//r
                (115, vec![1, 1, 1, 1, 1, 1]),//s
                (116, vec![1, 1, 1, 1, 1]),//t
                (119, vec![1, 1])//w
            ]);
            Self{
                commands: commands.clone()
            }
        }
        pub fn get_commands(&self) -> &HashMap<u8, Vec<u8>>{
            &self.commands
        }
    }
}