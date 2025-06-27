# Dancing Grandpa - Repository Prompt

The `dancing-grandpa` repository contains the code for a "baby entertainment system" called "Dancing Grandpa".
The system is written in idiomatic Rust and based on the Bevy engine.
The repository root is a Cargo workspace containing three packages.
The `website` binary package contains the code to run the system as a single-page web app.
The `desktop` binary package contains the code to run the system as a desktop application.
The `common` library package contains code that is shared between both applications
The system loads a set of image files and an audio file, according to a config file.
The config file also contains a description of how the images should be animated and the tempo of the audio file.
The system shows a number of copies of the animated image, and plays the audio file, using the provided tempo to time the animation.
When the audio file has completed, the system plays silence for a few seconds while fading out the images, and then restarts the sequence.
