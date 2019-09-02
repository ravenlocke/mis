# mis: Random search for maximal independent sets
`mis` is a tool for finding maximal independent sets with either (1) as many members as possible, 
or (2) as few members as possible. The rest of this document will go into how to install and use
`mis`, as well as giving some example use-cases where this is useful.

## Documentation

### Installation
There are two ways to obtain an `mis` installation. 

#### Use a pre-compiled binary
If you are using OSx or a linux x86_64 bit system, then this repository contains pre-compiled binaries 
in the `binaries` folder.

<pre>
# 1. Download the repository
git clone https://github.com/Ravenlocke/mis.git
# 2. Change into the folder containing the relevant binary
cd mis/binaries/OSx/x86_64 # OR cd mis/binaries/linux/x86_64
# 3. Move the binary to your path.
sudo mv mis /usr/bin
# 4. Confirm installation
mis --help
</pre>

#### Compile your own binary
The only pre-requisite is an existing Rust installation (instructions [here](https://www.rust-lang.org/tools/install)). 

<pre>
# 1. Download the repository
git clone https://github.com/Ravenlocke/mis.git
# 2. Change into the mis directory
cd mis
# 3. Compile
cargo build --release
# 4. Move the binary to your path.
sudo mv ./target/release/mis /usr/bin
# 5. Confirm installation
mis --help
</pre>

This will create a folder, `target/release/`, containing the compiled `mis` binary. 

Note that neither **installation methods put `mis` on the `PATH`**. 

---

### Usage
This tool, at present, only takes edge list inputs (aiming to support further formats in the future). 
An edge list is simply a list of `node_a node_b <optional properties>`, for example:

<pre>
A B {'weight': 3}
B C {'weight': 4}
</pre>
or 
<pre>
A B
B C
</pre>

The simplest usage is to run only passing the graph file, which will return the largest maximal independent 
set obtained over 10,000 random samplings using as many threads as cores available.

<pre>
mis -g &lt;edge_list&gt;
</pre>

The number of threads and random samplings to use can be specified using the `-t` and `-n` options respectively.

<pre>
mis -g &lt;edge_list&gt; -n 1000000 -t 4
</pre>

In order to return the smallest maximal independent set found, pass the `-s` flag.

<pre>
mis -g &lt;edge_list&gt; -n 1000000 -t 4 -s
</pre>

---

### Examples
#### Trivial example
Consider a trivial example of a network `A <-> B <-> C`. In this case, the largest maximal independent set is `{A, C}`, 
and the smallest maximal independent set is `{B}`. We can derive these with `mis` using the edge list in 
`examples/simple_edgelist.txt`

<pre>
$ mis -g examples/simple_edgelist.txt -n 1000000 -t 4
{"size":2,"members":["A","C"]}

$ mis -g examples/simple_edgelist.txt -n 1000000 -t 4 -s
{"size":1,"members":["B"]}
</pre>

The output here is a valid JSON dictionary containing two keys:
* `members`: the members of the largest / smallest maximal independent set.
* `size`: the number of members in the largest / smallest maximal independent set.

#### Larger example
The edge list `examples/test_edgelist.txt` contains 8,989 edges and 1,000 nodes.

<pre>
$ time ./target/release/mis -g test_edgelist.txt -n 1000000 -t 4
{"size":111,"members":["497","443","973","97","531","912","205","740","977","900","367","800","697","810","831","190","923","23","745","796","252","228","589","60","709","821","344","481","355","695","683","425","966","820","986","719","530","323","747","462","388","14","39","287","36","81","117","640","906","649","921","509","93","558","192","280","334","960","134","480","412","30","337","761","896","115","674","70","936","263","797","947","875","407","310","603","147","399","232","437","454","607","19","642","56","597","660","837","985","155","465","914","834","778","438","309","176","872","171","844","284","5","362","239","546","591","788","3","565","713","539"]}

real	1m27.883s
user	5m43.666s
sys	0m0.633s

$ time ./target/release/mis -g test_edgelist.txt -n 1000000 -t 4 -s
{"size":80,"members":["970","231","459","907","714","358","109","435","516","725","977","834","929","268","153","533","60","185","893","746","832","824","392","667","316","855","595","568","244","166","13","671","12","332","431","891","294","143","801","874","993","640","734","678","790","480","641","48","35","612","123","850","301","511","408","777","282","702","356","546","470","693","785","85","936","574","381","338","701","481","955","764","200","16","258","34","598","735","539","101"]}

real	1m27.940s
user	5m42.664s
sys	0m0.627s
</pre>

This demonstrates the power of `mis`; the tool is rapid, with 1,000,000 maximal independent networks sampled 
on a MacBook pro with 2.7 GHz Intel Core i5 in less than two minutes.
