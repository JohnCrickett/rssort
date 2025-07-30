# Rust Sort
Rust solution to Coding Challenges build your own sort.

## Step 0
Set up the testing by running":

```bash
curl https://www.gutenberg.org/cache/epub/132/pg132.txt -o test.txt
tr -s '[[:punct:][:space:]]' '\n' < test.txt |sed '/^[0-9]/d' > words.txt
```

## Step 1
```bash
./target/release/rssort words.txt | uniq | head -n5
A
ACTUAL
AGREE
AGREEMENT
AND
```

## Step 2
```bash
./target/release/rssort -u words.txt | head -n5

```

## Step 3
Quicksort
```bash
./target/release/rssort --qsort words.txt | uniq | head -n5
A
ACTUAL
AGREE
AGREEMENT
AND
``````

Mergesort
```bash
./target/release/rssort --mergesort words.txt | uniq | head -n5
A
ACTUAL
AGREE
AGREEMENT
AND
```

## Step 4
```bash
./target/release/rssort --random-sort words.txt | uniq | head -n5
instructions
hearted
any
Chinese
was
```