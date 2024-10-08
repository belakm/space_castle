# Space game :)

I'm open sourcing it due to time constraints and lack of a client.\
This project was started to learn Solana Dev and to explore the possibilities of creating a Travian-like game on blockchain.

## So what is it?

It features minimum viable features of a Travian-like game: users, bases, buildings, units, fighting, resources and global trade.

Read the (https://docs.google.com/document/d/1csSxpym-yWUFjzpSv7CLkAp7P2HOJqIG0l1UoCzg5L8/edit?usp=sharing)[Design document on Google Docs].

Check the (https://www.figma.com/design/8jx4zlPcKxEZH54dXd3yIw/Space-game)[Figma wireframes]

## Technically

It uses Anchor framework (0.30) and connects to Solana network. All objects are on-chain and program is fully on-chain.
It comes with tests for all endpoints.

## Client

Go ahead, build one :P\
Check the example in (https://github.com/belakm/space-client)[space-client] project.

## Passing tests:

These give a good enough overview of what features are included. Do check the tests themselves for usage.

[Test]: 💰 Mints\
  ✔ Intergalactic Tender (IGT) + metadata - main currency (392ms)\
  ✔ Metal (rMETL) + metadata - ingame resource (405ms)\
  ✔ Crystal (rCRYS) + metadata - ingame resource (405ms)\
  ✔ Chemicals (rCHEM) + metadata - ingame resource (405ms)\
  ✔ Fuel (rFUEL) + metadata - ingame resource (405ms)

[Test]: 🤠 Player

  ✔ Creating a new player (384ms)\
  ✔ New player gets a small amount of IGT tokens\
  ✔ Player resource token accounts initialization - this must be done in separate instructions due to size constraints (1619ms)\
  ✔ Player can't have a name longer than 32 characters

[Test]: 🪐 Planet

✔ First planet can be claimed for free (377ms)\
✔ First free planet be claimed only once\
✔ Can't claim a planet where there is no planet\
✔ Only planet without an owner can be claimed as free\
✔ Claims a planet with another user (required for tests that follow) (371ms)\
✔ Claiming a planet adds a token amount of IGT and resources to Player Cache\
✔ Player Cache can be claimed - IGT is deposited into wallet and resources into token accounts (827ms)\
✔ Planet harvesting, this grants player IGT and resources (405ms)\
✔ Planet can only be harvested by its owner

[Test]: 🏰 Buildings

✔ Planet starts with basic buildings - Planetary Capital, Shipyard and either: Crystal Labs, Metal Industry, Chemical Refinery\
✔ New buildings can be built on the planet (401ms)\
✔ New buildings are paid with resources\
✔ Buildings can be upgraded, this increases their level (401ms)\
✔ Upgrading a building costs resources\
✔ Buildings can be changed into another building type (389ms)\
✔ Changing a building halves its level\
✔ Changing a building costs resources

[Test] 💱 Market pool - a global automated market and market maker

✔ Market pool initialization (372ms)\
✔ Market pool can be filled with resources and IGT (2030ms)\
✔ All tokens are interchangable on the Market pool (IGT, rMETL, rCRYS, rCHEM, rFUEL) (8150ms)

[Test]: 🚀 Fleet

✔ Moving a fleet to new coordinates (371ms)\
✔ Moving a fleet costs resources\
✔ Fleet cant move where another fleet is present\
✔ Fleet cant move where a non-owned planet is\
✔ Creating a fleet (383ms)\
✔ Fleet cannot be created if another fleet is on that planet\
✔ Fleet can't move where another fleet is present\
✔ Only owner of the fleet can move it around

[Test]: ⚔️ Battle

✔ Fleet can attack another fleet (754ms)\
✔ Winner of the battle is granted plunder\
✔ Winner, loser or both lost some ships in the conflict\
✔ Fleet cant attack where there is not fleet\
✔ Fleet cant attack a planet as it would a fleet (that action is called planet invasion)

42 passing (24s)
