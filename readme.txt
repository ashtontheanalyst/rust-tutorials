Tutorial is provided by https://www.youtube.com/watch?v=rQ_J9WH6CGk

Running some code in the terminal:
   - Go to the /code/rust/rust-tutorials directory
   - Make a new directory for the code you wanna make like 'helloRupert'
   - Make a file called 'helloRupert.rs'
   - Put your code in that file
   - In the terminal run:
   rustc helloRupert.rs
   - This will make an executable, compiled file, to run it do this:
   - ./helloRupert
   - Output diplayed in terminal


Running all those commands takes a lot of time so instead make a bash function:
   - vim ~/.bashrc
   - Scroll to the very bottom and click 'i' to start typing
   - Enter the function you want, in this case it's:
# This is for code development in rust
mr() {
  if [ -z "$1" ]; then
    echo "Usage: mc <name>"
    return 1
  fi
  cd ~/code/rust/rust-tutorials && mkdir "$1" && cd "$1" && touch "$1.rs"
}
   - Click 'esc' key
   - Type :wq to save and exit 
   - In your terminal, source the file by doing:
   source ~/.bashrc
