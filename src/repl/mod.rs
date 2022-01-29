use crate::solver_interface::*;
use crate::solver_interface::error::TyError;
use std::io::{Write, BufRead};

enum RPLCommand {
    LoadFile(String),
    AddConstraint(String),
    Remove(String),
    RunInference,
    Clear,
    Exit,
    Help,
}

impl RPLCommand {
    pub fn parse(mut msg: String) -> Result<RPLCommand, ExecError> {
        use lazy_static::lazy_static;
        use regex::*;
        lazy_static! {
            static ref LOAD_FILE: Regex = Regex::new(r":load .+").unwrap();
            static ref ADD_CONSTRAINT: Regex = Regex::new(r":add .+").unwrap();
            static ref RUN_INFERENCE: Regex = Regex::new(r":infer").unwrap();
            static ref EXIT: Regex = Regex::new(r":exit").unwrap();
            static ref CLEAR: Regex = Regex::new(r":clear").unwrap();
            static ref REMOVE: Regex = Regex::new(r":rm ").unwrap();
            static ref HELP: Regex = Regex::new(r":help").unwrap();
        }
        msg = msg.trim().to_string();
        if LOAD_FILE.is_match(&msg) {
            let f: Vec<_> = msg.split(":load ").collect();
            Ok(RPLCommand::LoadFile(f[1].to_string()))
        } else if ADD_CONSTRAINT.is_match(&msg) {
            let c: Vec<_> = msg.split(":add ").collect();
            Ok(RPLCommand::AddConstraint(c[1].to_string()))
        } else if RUN_INFERENCE.is_match(&msg) {
            Ok(RPLCommand::RunInference)
        } else if REMOVE.is_match(&msg) {
            let c: Vec<_> = msg.split(":rm ").collect();
            Ok(RPLCommand::Remove(c[1].to_string()))
        } else if CLEAR.is_match(&msg) {
            Ok(RPLCommand::Clear)
        } else if EXIT.is_match(&msg) {
            Ok(RPLCommand::Exit)
        } else if HELP.is_match(&msg) {
            Ok(RPLCommand::Help)
        } else {
            Err(ExecError::UnknownCommand)
        }
    }
}

enum ExecError {
    FileNotFound,
    Parsing(ParseErr),
    EngineErr(TyError),
    UnknownCommand,
}

fn helper(output: &mut std::io::Stdout) -> Result<(), ExecError> {
    output.write_all(b":load [file] load the constraint file\n").unwrap();
    output.write_all(b":add [constraint] load the constraint\n").unwrap();
    output.write_all(b":infer apply the inference algorithm\n").unwrap();
    output.write_all(b":clear remove all the loaded constraints\n").unwrap();
    output.write_all(b":rm [index] remove the constraint at index\n").unwrap();
    output.write_all(b":exit\n").unwrap();
    output.flush().unwrap();
    Ok(())
}

fn load_file(state: &mut BlackBoard, file: &str) -> Result<(), ExecError> {
    use std::fs::File;
    let path = std::fs::canonicalize(file).map_err(|_| ExecError::FileNotFound)?;
    let file = File::open(path).map_err(|_| ExecError::FileNotFound)?;
    for line in std::io::BufReader::new(file).lines() {
        let line = line.map_err(|_| ExecError::FileNotFound)?;
        add_constraint(state, &line)?;
    }
    Ok(())
}

fn add_constraint(state: &mut BlackBoard, constraint: &str) -> Result<(), ExecError> {
    let constraint = Parse::parse(constraint).map_err(ExecError::Parsing)?;
    state.add_constraint(constraint);
    Ok(())
}

fn infer(state: &mut BlackBoard) -> Result<(), ExecError> {
    let step = state.get_constraints().clone();
    match state.infer() {
        Err(e) => {
            state.set_constraints(step);
            Err(ExecError::EngineErr(e))
        }
        Ok(_) => Ok(()),
    }
}

fn clear(state: &mut BlackBoard) -> Result<(), ExecError> {
    state.clear();
    Ok(())
}

fn remove(state: &mut BlackBoard, index: &String) -> Result<(), ExecError> {
    state.rm_constraint(index.parse().unwrap());
    Ok(())
}

pub fn repl() {
    let mut output = std::io::stdout();
    let input = std::io::stdin();
    let mut state = BlackBoard::new();
    loop {
        // Read
        output.write_all(b"<>").unwrap();
        output.flush().unwrap();
        let mut buffer = String::new();
        input.read_line(&mut buffer).unwrap();
        // Execute
        let result = match RPLCommand::parse(buffer.clone()) {
            Ok(RPLCommand::AddConstraint(cnst)) => add_constraint(&mut state, &cnst),
            Ok(RPLCommand::Remove(index)) => remove(&mut state, &index),
            Ok(RPLCommand::LoadFile(file)) => load_file(&mut state, &file),
            Ok(RPLCommand::RunInference) => infer(&mut state),
            Ok(RPLCommand::Clear) => clear(&mut state),
            Ok(RPLCommand::Help) => helper(&mut output),
            Ok(RPLCommand::Exit) => break,
            Err(e) => Err(e),
        };
        match result {
            Ok(_) => {}
            Err(e) => match e {
                ExecError::FileNotFound => {
                    output.write_all(b"[ERR] File not found\n").unwrap();
                    output.flush().unwrap();
                }
                ExecError::Parsing(err) => {
                    output.write_all(err.to_string().as_bytes()).unwrap();
                    output.write_all(b"\n").unwrap();
                    output.flush().unwrap();
                }
                ExecError::EngineErr(err) => {
                    output.write_all(err.to_string().as_bytes()).unwrap();
                    output.write_all(b"\n").unwrap();
                    output.flush().unwrap();
                }
                ExecError::UnknownCommand => {
                    output.write_all(b"[ERR] Unknown command\n").unwrap();
                    output.flush().unwrap();
                }
            },
        }

        // Print
        output.write_all(b"Current state:\n").unwrap();
        output.flush().unwrap();
        state.get_constraints().iter().enumerate().for_each(|(i, c)| {
            let msg = format!("[{}] {}", i, c.to_string());
            output.write_all(msg.to_string().as_bytes()).unwrap();
            output.write_all(b"\n").unwrap();
            output.flush().unwrap();
        });
    }
}
