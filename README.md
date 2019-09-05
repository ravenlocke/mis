[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.3384133.svg)](https://doi.org/10.5281/zenodo.3384133)

# mis: Random search for maximal independent sets
`mis` is a tool for finding maximal independent sets with either (1) as many members as possible, 
or (2) as few members as possible. The rest of this document will go into how to install and use
`mis`, as well as giving some example use-cases where this is useful. 

If you do use this tool, please support us by citing the initial release:

> David J. Skelton. (2019, September 2). mis: Random search for maximal independent sets (Version 0). Zenodo. http://doi.org/10.5281/zenodo.3384133


## Documentation

### Installation
There are two ways to obtain a `mis` installation. 

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
This section will give a few examples of using `mis`, with some speed comparisons to Python. These comparisons are not entirely direct, as all timings of `mis` include the parsing of the edge list, but in Python only the iteration of random maximal independent sets.

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


#### Random GNM network example
The edge list `examples/gnm_edgelist.txt` contains 1,000 nodes and 864 edges. This was generated using `networkx`'s `gnm_random_graph` function.

```
$ time mis -g examples/gnm_edgelist.txt -t 4 -n 1000000
{"size":441,"members":["317","356","787","581","7","145","524","693","660","306","156","712","160","752","500","766","163","663","388","243","353","554","293","338","803","587","633","411","814","294","280","954","108","595","805","26","862","990","871","883","927","509","755","547","270","363","602","278","87","359","611","906","838","282","797","174","694","345","404","573","579","369","413","236","17","971","901","702","970","170","657","714","879","892","21","457","676","867","972","424","43","988","349","229","671","105","786","30","291","788","314","512","118","487","768","368","374","75","810","39","492","203","281","308","610","352","648","531","585","677","619","651","572","90","637","746","834","836","942","780","76","303","638","329","877","614","272","193","344","471","268","709","719","288","201","789","12","290","49","815","52","540","179","536","659","860","264","74","812","273","171","194","382","884","6","489","926","42","730","111","418","184","566","407","16","949","891","734","109","852","853","904","333","920","711","77","227","700","258","187","367","443","122","606","987","757","931","89","517","292","313","1","609","507","644","684","873","202","856","821","136","824","277","250","8","181","733","737","759","480","469","192","62","486","905","0","393","592","798","148","708","846","791","897","98","882","580","126","608","692","688","5","357","440","123","247","206","37","316","620","984","154","777","176","100","431","470","510","778","131","14","924","137","263","939","332","647","164","446","35","15","24","322","415","155","583","146","497","428","240","978","948","55","739","337","365","991","518","523","28","150","334","625","704","506","732","474","253","754","675","666","128","260","416","261","432","870","839","20","539","626","973","375","809","138","244","745","325","817","251","197","955","784","82","600","213","31","773","863","279","300","36","832","371","259","597","630","996","420","888","993","615","29","319","120","741","664","191","97","707","384","961","96","311","899","845","820","189","310","866","340","462","401","230","175","312","161","717","165","220","508","654","494","269","47","207","297","681","837","771","182","466","451","459","71","634","373","674","408","233","69","868","912","999","88","909","636","221","796","829","121","869","139","448","204","478","224","186","706","532","683","855","113","844","25","235","521","886","975","928","178","849","925","550","364","848","691","434","559","231","149","793","964","567","890","520","45","232","116","696","162","498","449","392","27","919","438","133","641","385","177"]}

real	1m5.127s
user	4m13.521s
sys	0m0.521s
```

This demonstrates the power of `mis`; the tool is rapid, with 1,000,000 maximal independent networks sampled 
on a MacBook pro with 2.7 GHz Intel Core i5 in less than two minutes. Using Python, multiprocessing over four logical cores, this took nearer 30 minutes. Note that this kind of speed increase may not always be seen (depending on graph topology), but nevertheless demonstrates `mis` is capable of being substantially faster. Further, the algorithm for maximum independent set approximation in `networkx` had 417 members, showing an example where a random search is actually superior. 

---

### Why is this relevant?
My field of research is enzyme discovery, and there are two situations where this tool would be useful.

#### Building a commercial portfolio
Commercially, intellectual property on enzymes is often subject to restrictions on how similar they are (known as identity). If we represent this as a network, where two nodes are connected when they share identity ≥ a given threshold, then building the largest portfolio possible is equivalent to searching for the maximum independent set. While this problem remains intractable, using a tool such as `mis` will help build as substantial a portfolio as possible.

#### Testing a sample of enzymes from sequence space
Another use would be when testing the properties of a set of *enzymes*. In this instance, again, we can connect two nodes in a network when they share identity ≥ a given threshold where their acctivity would be likely to be highly similar. We could then obtain an independent set to test, but the smallest one in which the whole network is represented. Thus, using `mis` with the `-s` flag would give a small maximal independent set that achieves this.
