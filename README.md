# Seed Phrase - Raid 5

Split your 24 word phrase across three sheets, so you have 
3x 12 words.  
One sheet can be lost or stolen without you having to worry.  

So, basically like RAID 5.  
Except this has nothing to do with disks.  
This is only using the binary XOR operation to generate redundancy.

## 🌿 Disclaimer
I am not responsible for anything,
you are responsible for everything,
blah blah blah.

## 👎🏼 The wrong Raid 5
When you google for "wallet seed phrase raid 5", there is this very
bad idea of splitting your phrase into three blocks, and
then storing
- Sheet A: 1,2
- Sheet B: 2,3
- Sheet C: 3,1

This is not Raid 5 and way less secure.  
With this method you would store 16 of 24 words.
An attacker would only have to guess 8.

You would provide him with 4 more words than neccessary.
4 more word results in only 0,000000000005684342% time requirement.

__Do not give an attacker 99.999999999994315658% of the solution.__



## ✅ Potential Feature Considerations

__Dropping one word per sheet__  
This would mean one has to brute force about
2048 * 2048 = 4194304 = 2 ** 22 possibilities to restore from two sheets.  
This would result in a restoration duration of still less than a second.  

Why doing it then?  
When having stolen one sheet, one had to guess the missing 12 plus 2 words.  
Meaning having to check 2 ** 132 * 2 ** 22 = 2 ** 154 Words.  
That is the difference between  
`5444517870735015415413993718908291383296` and
`22835963083295358096932575511191922182123945984`.  

If you would have been able to guess the missing 12 words in
ten seconds (!),
it would now take you 485 days 10 hours 50 minutes and 40 seconds.  

When working on a exponential scale, every bit makes a difference.

__Adding a password__  
One could use a password and generate a hash of 264bit.  
All words would be xor-ed with this hash.  
These new words can now be used to generated block A, B and X.  

If one has to restore the word list, one could restore A, B and X
as usual and now the password is required to generate the
264bit hash, and xor the word list back to normal.

This would be much more safe than dropping one word per sheet,
but it also requires the user to remember it.

__More ideas or whishes? - Let me know!__


## 🧮 How does it work?
Your 24 word will get divided into block A and block B,
with 12 words each.  
Then there will be block X generated as redundancy.  
X = A xor B

You enter your 24 words (reading this should make you _REEEEALLY_ uncomfortable).  
Then it will be split into part A, part B and a redundancy part X.

- If you lose X, you still have A and B.  
- If you lose A, with X you can restore B.  
- If you lose B, with X you can restore A.  


## 🔑 How can I do it securely?
_Paranoia mode_: _on_ 🕵️‍♀️

Do the following steps and you should be fine:  

__⚠️ #0 Do not trust this repository__  
Do not trust this code or this guide.  
Check the code for yourself.
If you are not a computer scientist, ask one you trust.


__🛡 #1 Get Tails__  
When you run the code to generate redundancy,  
you want to leave not a single bit of information when running this program.  
You might know the Tor Browser. Tails is from the same people.

Download it and install it to an USB stick.  
[Tails](https://tails.boum.org/index.de.html)


__💻 #2 Run Tails__  
Boot from the USB stick.  
You have to trust the hardware. If there is
a disk with malware (including the Windows OS), that's fine.

You need to set a root password.  
[Guide](https://tails.boum.org/doc/first_steps/welcome_screen/administration_password/)

When launching, please connect to the Tor network, as we have
to fetch dependencies.

__🍍 #3 Get Rust__  
To generate this program from its source code, you need
the Rust compiler.

Please fire up the root terminal.  
Install Rust's packet manager cargo with
```
apt-get install cargo
```

__☕️ #4 Compile the program__  
Now, clone the code and compile it.
```bash
git clone https://github.com/julianbuettner/seed-phrase-raid-5.git
cd seed-phrase-raid-5
cargo build
```

__📡 #5 Secure your environment__  
_Paranoia mode_: _supercharged_
- Make sure your screen and keyboard are not seen
  - Move the screen away from any windows.  
  - Consider potential reflections.
  - Do not execute the process outdoors where someone might see you.  
    Also, spy satellites have really high resolutions these days.  
- Make sure the hardware you are using can be trusted.
    - Check for potential installed bugs or keyloggers.
- Make sure no one can record your keyboard clicks, processor ticks, etc.

_Very important:_
- Disable any possible way for the computer to connect to the internet.  

__🔐 #6 Run the Program__  

Double check all points of `#5 Secure your environemnt`.

Now run
```bash
cargo run
```
and follow the instructions.


__🍕 #7 Verify the Restoration__  
Pretend you lost the words from block A, and check if
the program can restore the words.

Now do the same as if you lost block B.

You do not have to check the loss of X,
as A + B is your original word list.


## 📝 A little bit of background (and math).
The words are from Bitcoin's Improvement Proposal BIP39.  
[Ian Coleman - BIP39](https://iancoleman.io/bip39/).

Word 0 is `abandon`, and represents eleven zerobits.  
`00000000000`.  
Word 2047 is `zoo`, and represents eleven ones.  
`11111111111`.

This structure allows to xor two words and obtaining a third one.  
This is known as creating parity.

The following rules apply to the xor operation.

`A xor B = B xor A`  
`(A xor B) xor B = A`  
`(A xor B) xor A = B`  


24 words have 264 bits, with a little bit of
redundancy in itself.  
The information contained is 256 bits.  
The entropy is 256bit / 264bit = 0.97.

When having one sheet, one has 132bit and 128bit information.  
To bruteforce the remaining 128bit, one has to calculate up to  
`340282366920938463463374607431768211456` Hashes.  
To be honest, I have no clue how secure this actually is.  
(Could someone please calculate how long this would take for different super computers?)

If someone steals one of your sheets, move your funds, get new keys.


## 🥕 Usage Examples
Here an example of creating an redundancy block:

```
$ cargo run
==================
Seed Phrase Raid 5
==================

 1: Create redundancy
 2: Restore from redundancy

What do you want to do? > 1

=================
Create redundancy
=================

Enter word 01 > patient
Enter word 02 > wall
Enter word 03 > rural
Enter word 04 > drink
Enter word 05 > sleep
Enter word 06 > school
Enter word 07 > scatter
Enter word 08 > twin
Enter word 09 > sibling
Enter word 10 > jeans
Enter word 11 > panda
Enter word 12 > frog
Enter word 13 > believe
Enter word 14 > bright
Enter word 15 > major
Enter word 16 > bonus
Enter word 17 > autumn
Enter word 18 > initial
Enter word 19 > regular
Enter word 20 > soul
Enter word 21 > weird
Enter word 22 > baby
Enter word 23 > ecology
Enter word 24 > average

== Block A ==
Word 01: patient
Word 02: wall
Word 03: rural
Word 04: drink
Word 05: sleep
Word 06: school
Word 07: scatter
Word 08: twin
Word 09: sibling
Word 10: jeans
Word 11: panda
Word 12: frog

== Block B ==
Word 01: believe
Word 02: bright
Word 03: major
Word 04: bonus
Word 05: autumn
Word 06: initial
Word 07: regular
Word 08: soul
Word 09: weird
Word 10: baby
Word 11: ecology
Word 12: average

== Block X ==
Word 01: remember
Word 02: turkey
Word 03: desk
Word 04: foil
Word 05: setup
Word 06: rebel
Word 07: input
Word 08: cave
Word 09: direct
Word 10: grit
Word 11: sunny
Word 12: fancy
```

Here a restoration from Block X and A:
```
$ cargo run
==================
Seed Phrase Raid 5
==================

 1: Create redundancy
 2: Restore from redundancy

What do you want to do? > 2
=============
Restore Block
=============

== Enter the words of block X ==
Enter word 01 > remember
Enter word 02 > turkey
Enter word 03 > deskkkkkkkk
Unknown word: "deskkkkkkkk"
Enter word 03 > desk
Enter word 04 > foil
Enter word 05 > setup
Enter word 06 > rebel
Enter word 07 > input
Enter word 08 > cave
Enter word 09 > direct
Enter word 10 > grit
Enter word 11 > sunny
Enter word 12 > fancy

== Enter the words of block A or B ==
Enter word 01 > patient
Enter word 02 > wall
Enter word 03 > rural
Enter word 04 > drink
Enter word 05 > sleep
Enter word 06 > school
Enter word 07 > scatter
Enter word 08 > twin
Enter word 09 > sibling
Enter word 10 > jeands
Unknown word: "jeands"
Enter word 10 > jeans
Enter word 11 > panda
Enter word 12 > frog

== The missing block (A or B) ==
Word 01: believe
Word 02: bright
Word 03: major
Word 04: bonus
Word 05: autumn
Word 06: initial
Word 07: regular
Word 08: soul
Word 09: weird
Word 10: baby
Word 11: ecology
Word 12: average
```