use rustix::termios::{ Termios, tcgetattr, tcsetattr, OptionalActions };
use crate::error::{ NebulaError, Result };

static PRIOR_TERM_MODE: spin::Mutex<Option<Termios>> = spin::Mutex::new(None);

pub fn enter_raw_mode() -> Result<()> {
    let stdout = unsafe { rustix::stdio::stdout() };
    let mut prior_term_mode = PRIOR_TERM_MODE.lock();
    if prior_term_mode.is_some() { return Ok(()); }

    let mut curr_term = tcgetattr(stdout).map_err(NebulaError::OsError).unwrap();
    let prior_term = curr_term.clone();
    curr_term.make_raw();
    tcsetattr(stdout, OptionalActions::Now, &curr_term).unwrap();

    // set prior terminal mode after the switch to raw mode
    *prior_term_mode = Some(prior_term);
    Ok(())
}

pub fn exit_raw_mode() -> Result<()> {
    let mut prior_term_mode = PRIOR_TERM_MODE.lock();

    if let Some(prior_term) = prior_term_mode.as_ref() {
        unsafe {
            tcsetattr(rustix::stdio::stdout(), OptionalActions::Now, prior_term)
                .map_err(NebulaError::OsError).unwrap();
        }
        *prior_term_mode = None;
    }
    Ok(())
}
