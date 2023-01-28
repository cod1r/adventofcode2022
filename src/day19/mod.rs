struct BluePrint {
    id: u32,
    ore_cost: u32,
    clay_cost: u32,
    obs_cost: (u32, u32),
    geode_cost: (u32, u32),
}
impl BluePrint {
    fn new(
        id: u32,
        ore_cost: u32,
        clay_cost: u32,
        obs_cost: (u32, u32),
        geode_cost: (u32, u32),
    ) -> BluePrint {
        BluePrint {
            id,
            ore_cost,
            clay_cost,
            obs_cost,
            geode_cost,
        }
    }
}

#[derive(Clone)]
enum RType {
    Ore(Resource),
    Clay(Resource),
    Obsid(Resource),
    Geode(Resource),
}

#[derive(Clone)]
struct Resource {
    count: u32,
    robots: u32,
}

impl Resource {
    fn new(count: u32, robots: u32) -> Resource {
        Resource { count, robots }
    }
}

#[derive(Clone)]
struct Node {
    ore: RType,
    clay: RType,
    obsid: RType,
    geode: RType,
    minute: u32,
}
impl Node {
    fn new(ore: RType, clay: RType, obsid: RType, geode: RType, minute: u32) -> Node {
        Node {
            ore,
            clay,
            obsid,
            geode,
            minute,
        }
    }
    fn update(&mut self) {
        match &mut self.ore {
            RType::Ore(or) => {
                or.count += or.robots;
            }
            _ => unreachable!(),
        }
        match &mut self.clay {
            RType::Clay(cr) => {
                cr.count += cr.robots;
            }
            _ => unreachable!(),
        }
        match &mut self.obsid {
            RType::Obsid(obr) => {
                obr.count += obr.robots;
            }
            _ => unreachable!(),
        }
        match &mut self.geode {
            RType::Geode(gr) => {
                gr.count += gr.robots;
            }
            _ => unreachable!(),
        }
        self.minute += 1;
    }
}
fn dfs<const E: u32>(bp: &BluePrint, node: Node, stage: u8, minute: u32) -> u32 {
    if minute == E {
        return match node.geode {
            RType::Geode(gr) => {
                //println!("{}", gr.count);
                gr.count
            },
            _ => unreachable!(),
        };
    }
    let mut ans = 0;
    let mut node2 = node.clone();
    node2.update();
    ans = ans.max(dfs::<E>(bp, node2, stage, minute + 1));
    match stage {
        1 => {
            let mut node3 = node.clone();
            node3.update();
            let or = match node3.ore {
                RType::Ore(or) => or,
                _ => unreachable!(),
            };
            // getting another ore robot
            if or.count >= bp.ore_cost {
                ans = ans.max(dfs::<E>(
                    bp,
                    Node::new(
                        RType::Ore(Resource::new(or.count - bp.ore_cost, or.robots + 1)),
                        node3.clay.clone(),
                        node3.obsid.clone(),
                        node3.geode.clone(),
                        node3.minute,
                    ),
                    1,
                    node3.minute,
                ));
            }
            // getting clay robot
            if or.count >= bp.clay_cost {
                let cr = match &node3.clay {
                    RType::Clay(cr) => cr,
                    _ => unimplemented!(),
                };
                ans = ans.max(dfs::<E>(
                    bp,
                    Node::new(
                        RType::Ore(Resource::new(or.count - bp.ore_cost, or.robots)),
                        RType::Clay(Resource::new(cr.count, cr.robots + 1)),
                        node3.obsid,
                        node3.geode,
                        node3.minute,
                    ),
                    2,
                    node3.minute,
                ));
            }
        }
        2 => {
            let mut node3 = node.clone();
            node3.update();
            match &node3.ore {
                RType::Ore(or) => match &node3.clay {
                    RType::Clay(cr) => {
                        let obr = match node3.obsid {
                            RType::Obsid(obr) => obr,
                            _ => unimplemented!(),
                        };
                        // getting obsidian robot
                        if or.count >= bp.obs_cost.0 && cr.count >= bp.obs_cost.1 {
                            ans = ans.max(dfs::<E>(
                                bp,
                                Node::new(
                                    RType::Ore(Resource::new(or.count - bp.obs_cost.0, or.robots)),
                                    RType::Clay(Resource::new(cr.count - bp.obs_cost.1, cr.count)),
                                    RType::Obsid(Resource::new(obr.count, obr.robots + 1)),
                                    node3.geode,
                                    node3.minute,
                                ),
                                3,
                                node3.minute,
                            ));
                        }
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        3 => {
            let mut node3 = node.clone();
            node3.update();
            let or = match &node3.ore {
                RType::Ore(or) => or,
                _ => unreachable!(),
            };
            let obr = match &node3.obsid {
                RType::Obsid(obr) => obr,
                _ => unreachable!(),
            };
            let gr = match &node3.geode {
                RType::Geode(gr) => gr,
                _ => unreachable!(),
            };
            // getting geode
            if or.count >= bp.geode_cost.0 && obr.count >= bp.geode_cost.1 {
                ans = ans.max(dfs::<E>(
                    bp,
                    Node::new(
                        RType::Ore(Resource::new(or.count - bp.geode_cost.0, or.robots)),
                        node3.clay,
                        RType::Obsid(Resource::new(obr.count - bp.geode_cost.1, obr.robots)),
                        RType::Geode(Resource::new(gr.count, gr.robots + 1)),
                        node3.minute,
                    ),
                    4,
                    node3.minute,
                ));
            }
        }
        4 => {
            let gr = match &node.geode {
                RType::Geode(gr) => gr,
                _ => unreachable!()
            };
            println!("what: {} {}", gr.count, node.minute);
        }
        _ => unreachable!(),
    }
    ans
}
pub fn day19() {
    let input_str = include_str!("example.txt");
    let mut blueprints = input_str.trim().lines().map(|line| {
        let mut parts = line.split(' ');
        let num_colon = parts.nth(1).unwrap();
        let id = num_colon[..num_colon.len() - 1].parse::<u32>().unwrap();
        let ore_cost = parts.nth(4).unwrap().parse::<u32>().unwrap();
        let clay_cost = parts.nth(5).unwrap().parse::<u32>().unwrap();
        let obsidian_cost1 = parts.nth(5).unwrap().parse::<u32>().unwrap();
        let obsidian_cost2 = parts.nth(2).unwrap().parse::<u32>().unwrap();
        let geode_cost1 = parts.nth(5).unwrap().parse::<u32>().unwrap();
        let geode_cost2 = parts.nth(2).unwrap().parse::<u32>().unwrap();
        BluePrint::new(
            id,
            ore_cost,
            clay_cost,
            (obsidian_cost1, obsidian_cost2),
            (geode_cost1, geode_cost2),
        )
    });
    while let Some(bp) = blueprints.next() {
        let ans = dfs::<24>(
            &bp,
            Node::new(
                RType::Ore(Resource::new(0, 1)),
                RType::Clay(Resource::new(0, 0)),
                RType::Obsid(Resource::new(0, 0)),
                RType::Geode(Resource::new(0, 0)),
                0,
            ),
            1,
            0,
        );
        println!("{ans}");
        break;
    }
}
/*
 * problem is figuring out what robot to build and when
 * trying out every possibility takes way too long
 *
 * # of robots * minutes = # of thing
 *
 * end goal: how many geodes can we get
 *
 * with one ore robot, i can build 12 clay robots in 24 minutes.
 * the number of clay robots at any minute can be given with the equation:
 * # of ore robots * (curr min - 1) / clay cost
 * how much obsidian can we calculate for a given minute, given that we
 * would it make sense to alter from buying one robot to buying another type of
 * robot
 *
 * let's just focus on the primary goals
 * try to maximize geodes by getting geode robots
 * have to do that in 24 minutes but fast
 * our dfs function that tries to do every...
 * what happens if we just dfs but do it in a better way?
 *
 *
 * two main things we need to focus on.
 * for a certain minute, if we have enough to get a robot,
 * do we save to get another robot or get that robot
 *
 * if you can afford robots, you can dfs for one type of robot
 * and dfs for another type of robots and so on...
 * basically dfs for only the things you need.
 * if we can't afford a robot, we dfs as if we bought nothing
 * dfs probably won't work? idk...
 *
 *
 *
 * IDEA WE CAN DO A GRAPH APPROACH!!!
 * eh...
 * there is something deeper that I need to consider to remove some of the possible nodes or
 * outcomes
 * for example, we don't need many ore robots but we do need a lot of clay and obsidian robots.
 * we need to look at nodes that don't increase robots and save resources.
 */
