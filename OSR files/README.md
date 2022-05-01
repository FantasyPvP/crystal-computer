# Open Source Redstone
 written in python by a computer obsessed gcse student (my strongest subjects are computer science
and maths which given I'm starting this kind of project should be fairly obvious)

so yeah I decided to make a computer in python cos i got bored of imagining life using a computer from the last 10 years,
i figured that the solution was just to make my own in python

im using python [EDIT 01.05.22 im using rust now because its just superior] because it is the only language i am familiar with however if performance becomes an issue for a basic command line
operating system that i intend to make, which i suspect it will be it may be a smart idea to use c++ as i have some very basic knowledge
about it so i wouldnt need to start learning completely from scratch




my idea for the computer is that each memory register will be a python object that is defined / created on the initialisation of the program 
so a binary value (a list of integers [0-1]) can be stored in the variable and controlled by an object class
    (i could also use tuple when i assign values to memory so that i am not tempted to change individual bits but idk)

 all operations / arithmetic operations that this program will perform will be in binary and the entire system will run on a basic
    set of binary functions (for example, AND, NOT, OR, ADD, SUB, DIV) and so on, all of these functions will manipulate the values
    just like on real hardware
it would probably easier to do other arithmetic operations and things to do this however this could be implemented into a minecraft redstone computer
my friend may implement this for me if the architecture is feasable to implement as he is an expert on restone computers 
his channel link here: (https://www.youtube.com/c/TheDarkness344/videos)

My aim is to make this completely from the ground up, in a literal sense
In the past I've tried projects where I've started with advanced structures and tried to work my way down to lower level constructs
I've learned that this doesn't usually work and that I need to start bottom up with something low level and work my way up 

my long term goal (if i actually work on this project and not forget about it) is the following:

    - create a hardware system that can run binary commands (probably an 8 bit instruction set) and store 16 bit numbers using a 16 bit memory addressing system
    - create a basic command line that can run on the virtual hardware
    - implement an custom low level language (assembly) that can be written in a code editor and compiled for use on the vm
    - maybe make a version of this into a higher level language that can be compiled down to the assembly to make it easier to program more
        complex things 
    - implement a system for applications and running programs / saving them in storage
        - this will most likely use "ROM" files stored externally as text files full of binary
    - make this into a full application (im not gonna try doing a pyinstaller exe though as im using linux as my development platform so it would be impractical)
    - for the os create a spec that is easy to program for, efficient / scalable and can run on a Minecraft computer so someone can make use of the architecture
        to make something that can run a standard program library that I and anyone else who knows the spec can program for.

(2022 EDIT)

- ive spent the last few weeks learning rust (a low level programming language similar to c++ but with greater memory safety)
- i figured that it might be a good idea to start working on this project again and maybe with some of my friends from a coding club that i'm a part of.
- ive started work on a combined compiler application for a high level language and a low level / assembly language combined.
- once ive done this i'll work on making a virtual machine
