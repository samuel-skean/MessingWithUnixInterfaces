Try very large file sizes. Do any of them cause short writes when written to a terminal?
What happens if the user types something while this is writing something to the terminal? Can this program read that data later? (I think so)
Does it matter if the pseudo terminal was `dup`ed or separately opened?