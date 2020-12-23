/* Deep Deterministic Policy Gradient.
   Continuous control with deep reinforcement learning, Lillicrap et al. 2015
   https://arxiv.org/abs/1509.02971
   See https://spinningup.openai.com/en/latest/algorithms/ddpg.html for a
   reference python implementation.
*/
mod renderer;
use sdl2::pixels::Color;

use sandbox::env::Env;
use tch::{
    kind::{FLOAT_CPU, DOUBLE_CPU, INT64_CPU},
    nn,
    nn::OptimizerConfig,
    Device,
    Kind::Float,
    Tensor,
};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::renderer::Renderer;

// The impact of the q value of the next state on the current state's q value.
const GAMMA: f64 = 0.99;
// The weight for updating the target networks.
const TAU: f64 = 0.005;
// The capacity of the replay buffer used for sampling training data.
const REPLAY_BUFFER_CAPACITY: usize = 100_000;
// The training batch size for each training iteration.
const TRAINING_BATCH_SIZE: usize = 100;
// The total number of episodes.
//const MAX_EPISODES: usize = 100;
const MAX_EPISODES: usize = 500_000;
// The maximum length of an episode.
//const EPISODE_LENGTH: usize = 200;
const EPISODE_LENGTH: usize = 1000;
// The number of training iterations after one episode finishes.
//const TRAINING_ITERATIONS: usize = 200;
const TRAINING_ITERATIONS: usize = 200;

fn model(p: &nn::Path, nact: i64) -> Box<dyn Fn(&Tensor) -> (Tensor, Tensor)> {
    let critic = nn::linear(p / "cl", 512, 1, Default::default());
    let actor = nn::linear(p / "al", 512, nact, Default::default());
    let device = p.device();
    Box::new(move |xs: &Tensor| {
        let xs = xs.to_device(device);
        (xs.apply(&critic), xs.apply(&actor))
    })
}

pub fn main() {
    let mut rng: StdRng = SeedableRng::seed_from_u64(1);
    let mut env = Env::new("../../sandbox/data/gavle.json".to_string(), 1);
    let mut renderer = Renderer::new(env.scalex, env.scaley);
    //renderer.init();
    println!("action space: {}", env.action_space());
    println!("observation space: {}", env.observation_space());

    let num_obs = env.observation_space() as usize;
    let num_actions = env.action_space() as i64;

    let device = tch::Device::cuda_if_available();
    let vs = nn::VarStore::new(device);
    let model = model(&vs.root(), num_actions);
    let mut opt = nn::Adam::default().build(&vs, 1e-4).unwrap();

    let mut sum_rewards = Tensor::zeros(&[1], FLOAT_CPU);
    let mut total_rewards = 0f64;
    let mut total_episodes = 0f64;

    'running: for episode in 0..MAX_EPISODES {
        if renderer.quit() {
            break 'running;
        }
        let mut obs = Tensor::zeros(&[num_obs as _], FLOAT_CPU);
        env.reset(0);
        //dbg!(&obs);
        let mut total_reward = 0.0;
        for _ in 0..EPISODE_LENGTH {

            let (critic, actor) = tch::no_grad(|| model(&obs));
            let probs = actor.softmax(-1, Float);
            let actions = probs.multinomial(1, true).squeeze1(-1);

            //let (state, reward, done) = env.step(Vec::<i64>::from(&actions), 0)?;
            let (state, reward, done) = env.step(2, 0);

            /*
            sum_rewards += &reward;
            total_rewards += f64::from((&sum_rewards * &done).sum(Float));
            total_episodes += f64::from(step.is_done.sum(Float));


            let (state, reward, done) = env.step(action, 0);
            renderer.clear();
            renderer.render_line_strings(&env.line_strings.iter().collect(), Color::RGB(0, 255, 0), &env.agents.get(0).unwrap().position);
            renderer.render_points(&env.targets, Color::RGB(255, 0, 255), &env.agents.get(0).unwrap().position);
            renderer.render_rays(&env.agents.get(0).unwrap().rays, Color::RGB(0, 0, 255), &env.agents.get(0).unwrap().position);
            renderer.present();
            total_reward += reward;
            let state_t = Tensor::of_slice(&state).totype(Float);
            //state_t;
            //dbg!(&state_t);
            agent.remember(&obs, &actions.into(), &reward.into(), &state_t);
            */
            if done {
                break;
            }

            //obs = state_t;


        }

        println!("episode {} with total reward of {}", episode, total_reward);

        /*
        for _ in 0..TRAINING_ITERATIONS {
            agent.train(TRAINING_BATCH_SIZE);
        }

         */
    }
}