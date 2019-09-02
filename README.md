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
The edge list `examples/test_edgelist.txt` contains 8,989 edges and 1,000 nodes. This was generated using `networkx`'s `gaussian_random_partition` graph with `n = 1000`, `s = 20`, `v=5`, `p_in = 0.7`, and `p_out = 0.0002`.

<pre>
$ time mis -g test_edgelist.txt -n 1000000 -t 4
{"size":111,"members":["497","443","973","97","531","912","205","740","977","900","367","800","697","810","831","190","923","23","745","796","252","228","589","60","709","821","344","481","355","695","683","425","966","820","986","719","530","323","747","462","388","14","39","287","36","81","117","640","906","649","921","509","93","558","192","280","334","960","134","480","412","30","337","761","896","115","674","70","936","263","797","947","875","407","310","603","147","399","232","437","454","607","19","642","56","597","660","837","985","155","465","914","834","778","438","309","176","872","171","844","284","5","362","239","546","591","788","3","565","713","539"]}

real	1m27.883s
user	5m43.666s
sys	0m0.633s

$ time mis -g test_edgelist.txt -n 1000000 -t 4 -s
{"size":80,"members":["970","231","459","907","714","358","109","435","516","725","977","834","929","268","153","533","60","185","893","746","832","824","392","667","316","855","595","568","244","166","13","671","12","332","431","891","294","143","801","874","993","640","734","678","790","480","641","48","35","612","123","850","301","511","408","777","282","702","356","546","470","693","785","85","936","574","381","338","701","481","955","764","200","16","258","34","598","735","539","101"]}

real	1m27.940s
user	5m42.664s
sys	0m0.627s
</pre>

This demonstrates the power of `mis`; the tool is rapid, with 1,000,000 maximal independent networks sampled 
on a MacBook pro with 2.7 GHz Intel Core i5 in less than two minutes. The equivalent code in Python for this network using `networkx` and `multiprocessing` took 10min 39s. Further, the algorithm for maximum independent set approximation in `networkx` had 94 members, showing an example where a random search is superior.

#### ER example
The edge list `examples/er_edgelist.txt` contains 8,989 edges and 1,000 nodes. This was generated using `networkx`'s `gaussian_random_partition` graph with `n = 1000` and `p = 0.01`. 

Looking at just the larger maximal independent sets, we get the following results for 1,000,000 examples.

<pre>
time mis -g examples/er_edgelist.txt -n 1000000 -t 4 
{"size":267,"members":["980","987","632","615","684","872","811","641","928","819","135","782","320","446","695","709","31","861","917","726","681","586","24","150","261","943","406","608","777","590","578","935","257","995","923","609","343","840","576","560","991","53","251","774","758","280","912","337","321","275","77","137","860","331","821","182","715","634","303","890","885","807","465","242","290","903","64","103","175","799","880","357","439","342","225","792","757","745","914","329","896","179","67","305","846","864","852","937","837","415","435","178","623","295","544","350","859","875","255","78","183","59","532","667","277","551","680","768","36","28","983","165","783","39","270","633","328","15","123","932","844","919","849","272","262","990","382","177","503","568","922","221","524","496","962","660","388","802","570","772","104","839","822","384","986","747","947","5","927","143","504","622","58","674","449","624","217","592","754","497","186","269","906","827","0","181","535","430","740","727","396","422","742","309","728","195","42","311","32","112","106","675","387","476","136","315","286","198","788","696","804","784","2","323","756","127","665","317","650","960","395","631","877","452","153","281","985","580","542","853","273","992","486","128","253","930","789","712","938","561","162","87","161","900","595","34","703","817","976","668","82","629","694","56","948","445","942","553","961","760","232","464","37","131","338","491","478","587","814","88","38","440","689","850","231","116","44","970","475","404","70","902","549","720","65","508","282"]}

real	1m37.859s
user	6m11.041s
sys	0m0.710s
</pre>
