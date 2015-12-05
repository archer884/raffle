# raffle
Simple program for selecting raffle winners without replacement

Run like this:

    raffle <entrants> <winners>

...where `<entrants>` is the file containing the entries and `<winners>` is the path where you want the winners stored. The program stores the winners in that file and also reads whatever is already in that file so that a given person can't win twice.
