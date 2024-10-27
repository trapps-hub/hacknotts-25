# Queendoms: Long Live the Queens!

A game inspired by the LinkedIn Queen's puzzle that was coded in 24hrs during [HackNotts 24](https://hacknotts-24.devpost.com/). It's a procedurally generated version
 of Queens where each row, column and region must contain a queen, but they must not be adjacent! 
 _Fill the queendoms with queens to rule the queendoms!_

## Inspiration
I (Emily) feel an aversion to LinkedIn that I'm sure anyone who has used the service has experienced at least once. 
But one thing keeps me active: **the games**. 

There are games on LinkedIn; if you didn't already know, _get to know_. 
Four games, three of which are beautiful (sorry Pinpoint), #
and one rules over them all. 
**Queens** is a sudoku/chess derived game, where every row, 
column and colour region must have a queen, whilst also 
satisfying the [eight queens problem](https://www.geeksforgeeks.org/8-queen-problem/).

I love playing Queens, even though I often suck at it. 
It gives me the same satisfaction that Sudoku gives me but with 
a delightful repackaging that gives it novelty. But LinkedIn only gives you one puzzle a day! 
How will I ever satisfy my Queens cravings with such limitations?

That's why we created **Queendoms**. 

## What it does
Queendoms procedurally generates Queens boards for you to play to your heart's content! Here's a rundown of the features:

### Homebrew backtracking solver
We have a backtracking solver that checks the generated board only has one solution. This ensures that the generated board isn't too easy or requires guessing to solve correctly. Solving Queens is very interestingly the same problem complexity as solving Sudoku (NP-complete)! There's an interesting article to read about this [here](https://www.researchgate.net/publication/238205893_Complexity_and_Completeness_of_Finding_Another_Solution_and_Its_Application_to_Puzzles).

### Seed sharing and loading
Each board has an associated seed that can be copied and shared with your friends! Your friends would then be able to load the seed into their game and attempt the same board as you.

### Game timer
So you can compete against said friends, of course!

### Reactive design
The UI behaves very similarly to the LinkedIn interface - click to mark a slot as no queen, click again to place a queen, click _again_ to remove the queen and make the slot blank. If you place two queens that collide with eachother, the interface gets _angry_ and shows you how the queens collide using a shader.

## How we built it
We built this using **Godot** for the front end design and **Rust** for the back end logic implementation. 
We also used the _beautiful_ [Jacquard 24](https://fonts.google.com/specimen/Jacquard+24/about?query=Sarah+Cadigan-Fried) 
font to make our UI look extra scrumptious.

## Challenges we ran into
From the get go, the logic of this game is so tricky! It involves a lot of matrix manipulation and so we spent a lot of time working out the implementation - we generated our first valid board at midnight.

Procedural generation of course brings the joy of very unique errors that only pop up every once and a while (this is why its nice to have the seeds). 

... And then there's the fact that we coded it in Rust. 
"That wasn't my idea, I'd like to make that clear at this stage. Godot is also weird - kinda cool, but also weird. My Unity brain was quite confused at points. _What do you mean everything isn't inheriting everything?_" - Emily

_We've reached out to Daudi for comment._

## Accomplishments that we're proud of

We generated unique boards with only one solution! That is very cool! It took us a very long time and a lot of debugging so we're very pleased that we achieved our goal. We're also very pleased to have finished a working build with no (known) bugs!

The UI is also very pretty and has nice colours <3.

## What we learned

We learnt that incremental testing is very very important; we paid dearly everytime we skipped incremental testing. We learnt how to use Godot, and that if you believe hard enough Rust can be used in _every_ scenario.

We also learnt a lot about the logic behind the game; it's definitely a game that deserves more exploration by developers and puzzle enthusiasts alike!

## What's next for Queendoms

_Chaos has ruled the realm of Queendoms for the last 24 hours... order needs to be restored to the code so we can ensure the safety and security of the Queendoms._

Now that we have a working implementation of the standard game, the sky's the limit! We thought of plenty of experimental modes / features we could add to make the game even more interesting: 
- A board grid that expands with each successful solve - how far can you go?!
- More variation rules in the procedural generation algorithm to make funkier and harder maps; _what makes a difficult Queens board?_
- Saving the users' best time on a new board for improved gloating capabilities.
- Can each of the queens have different rules and characteristics? Every queen rules differently after all!
- In the code, each colour actually has a name, so yellow is the Farmlands: can we display these regions to the user in some way?
- Compile the game for different platforms, such as a web/Android application!


🌟 _LONG LIVE THE QUEENS!_ 🌟
