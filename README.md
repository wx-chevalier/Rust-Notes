[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![license: CC BY-NC-SA 4.0](https://img.shields.io/badge/license-CC%20BY--NC--SA%204.0-lightgrey.svg)][license-url]

<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/wx-chevalier/repo">
    <img src="https://ngte-superbed.oss-cn-beijing.aliyuncs.com/item/header.svg" alt="Logo" style="width: 100vw;height: 400px" />
  </a>

  <p align="center">
    <a href="https://ng-tech.icu/books/Rust-Notes"><strong>在线阅读 >> </strong></a>
    <br />
    <br />
    <a href="https://github.com/wx-chevalier">代码案例</a>
    ·
       <a href="https://github.com/wx-chevalier/Awesome-Lists">参考资料</a>

  </p>
</p>

<!-- ABOUT THE PROJECT -->

# Introduction | 前言

当我们需要编写接近实时高性能，稳健，并有足够开发效率的大程序时，譬如数据库、交易系统、大型桌面应用等，往往会首先选择 C 或者 C++。这种情况下我们往往有以下考量：

- 不需要自动垃圾回收(GC)，避免因为垃圾回收导致的程序中断，以及可能引发的错误或者麻烦。

- 有成熟完善的基础组件库，以保证开发效率。这一点正是 C 的不足，其虽然能保证高性能，但却需要重复造轮子；而 C++ 虽然内置了标准库，但是其却缺乏统一、完善的包管理器。

- 零开销抽象(Zero Cost Abstraction)，我们希望有合适的抽象来保证开发效率、可读与可维护性，但是这种抽象必须是没有运行时开销的。因此我们需要静态强类型多范式语言，编译器能够尽早地发现问题，并且在编译阶段即能自动地进行性能优化；譬如 C++ 中，编译器如果发现虚类 (Virtual Class) 没有真正被用到甚至会优化掉虚表 (Virtual Table)。

借鉴 [How Stylo Brought Rust and Servo to Firefox](http://bholley.net/blog/2017/stylo.html) 一文中的阐述，Mozilla, Google, Apple, 以及 Microsoft 等优秀公司开发的大型的 C/C++ 应用中，错误与漏洞从未停止，我们需要的是一门安全、高效、可扩展的语言。作为新语言，Rust 没有太多历史的包袱；但是 Rust 也并非一蹴而就，而是近 30 年的编程语言理论研究和实际软件工程的经验的集大成者：

- They borrowed Apple’s [C++ compiler backend](https://llvm.org/), which lets Rust match C++ in speed without reimplementing decades of platform-specific code generation optimizations.

- They leaned on the existing corpus of research languages, which contained droves of well-vetted ideas that nonetheless hadn’t been or couldn’t be integrated into C++.

- They included the _unsafe_ keyword - an escape hatch which, for an explicit section of code, allows programmers to override the safety checks and do anything they might do in C++. This allowed people to start building real things in Rust without waiting for the language to grow idiomatic support for each and every use case.

- They built a convenient [package ecosystem](https://crates.io/), allowing the out-of-the-box capabilities of Rust to grow while the core language and standard library remained small.

Rust 是为工业应用而生，并不拘泥于遵循某个范式( Paradigm )，笔者认为其最核心的特性为 Ownership 与 Lifetime；能够在没有 GC 与 Runtime 的情况下，防止近乎所有的段错误，并且保证线程安全(prevents nearly all segfaults, and guarantees thread safety )。Rust 为每个引用与指针设置了 Lifetime，对象则不允许在同一时间有两个和两个以上的可变引用，并且在编译阶段即进行了内存分配(栈或者堆)；Rust 还提供了 Closure 等函数式编程语言的特性、编译时多态(Compile-time Polymorphism )、衍生的错误处理机制、灵活的模块系统等。从应用层面来看，Mozilla 本身就是 Web 领域的执牛耳者，无论是使用 Rust 开发 Node.js 插件，还是 [Rust 默认支持 WebAssembly](https://parg.co/UPo)，都能很好地弥补目前笔者在进行 Web 前端 / Electron 客户端 / Node.js 计算模块的一些性能缺失。

## 背景

任何一门新技术的兴起，都是为了解决一个问题。自操作系统诞生以来，系统级主流编程语言，从汇编语言到 C++，已经发展了近 50 个年头，但依然存在两个难题：

很难编写内存安全的代码。

很难编写线程安全的代码。

这两个难题存在的本质原因是 C/C++属于类型不安全的语言，它们薄弱的内存管理机制导致了很多常见的漏洞。其实 20 世纪 80 年代也出现过非常优秀的语言，比如 Ada 语言。Ada 拥有诸多优秀的特性：可以在编译期进行类型检查、无 GC 式确定性内存管理、内置安全并发模型、无数据竞争、系统级硬实时编程等。但它的性能和同时期的 C/C++相比确实是有差距的。那个时代计算资源匮乏，大家追求的是性能。所以，大家都宁愿牺牲安全性来换取性能。这也是 C/C++得以普及的原因。

“Rust”这个名字包含了 GH 对这门语言的预期。在自然界有一种叫作锈菌（Rust Fungi）的真菌，这种真菌寄生于植物中，引发病害，而且号称“本世纪最可怕的生态病害”之一。这种真菌的生命力非常顽强，其在生命周期内可以产生多达 5 种孢子类型，这 5 种生命形态还可以相互转化，如果用软件术语来描述这种特性，那就是“鲁棒性超强”。可以回想一下 Rust 的 Logo 形状，像不像一个细菌？Logo 上面有 5 个圆圈，也和锈菌这 5 种生命形态相对应，暗示 Rust 语言的鲁棒性也超强。Rust 也有铁锈的意思，暗合裸金属之意，代表其系统级编程语言属性，有直接操作底层硬件的能力。此外 Rust 在字形组合上也糅合了 Trust 和 Robust，暗示了信任与鲁棒性。

# Nav | 关联导航

# About | 关于

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- ACKNOWLEDGEMENTS -->

## Acknowledgements

- [Awesome-Lists](https://github.com/wx-chevalier/Awesome-Lists): 📚 Guide to Galaxy, curated, worthy and up-to-date links/reading list for ITCS-Coding/Algorithm/SoftwareArchitecture/AI. 💫 ITCS-编程/算法/软件架构/人工智能等领域的文章/书籍/资料/项目链接精选。

- [Awesome-CS-Books](https://github.com/wx-chevalier/Awesome-CS-Books): :books: Awesome CS Books/Series(.pdf by git lfs) Warehouse for Geeks, ProgrammingLanguage, SoftwareEngineering, Web, AI, ServerSideApplication, Infrastructure, FE etc. :dizzy: 优秀计算机科学与技术领域相关的书籍归档。

## Copyright & More | 延伸阅读

笔者所有文章遵循[知识共享 署名 - 非商业性使用 - 禁止演绎 4.0 国际许可协议](https://creativecommons.org/licenses/by-nc-nd/4.0/deed.zh)，欢迎转载，尊重版权。您还可以前往 [NGTE Books](https://ng-tech.icu/books-gallery/) 主页浏览包含知识体系、编程语言、软件工程、模式与架构、Web 与大前端、服务端开发实践与工程架构、分布式基础架构、人工智能与深度学习、产品运营与创业等多类目的书籍列表：

[![NGTE Books](https://s2.ax1x.com/2020/01/18/19uXtI.png)](https://ng-tech.icu/books-gallery/)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/wx-chevalier/repo.svg?style=flat-square
[contributors-url]: https://github.com/wx-chevalier/repo/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/wx-chevalier/repo.svg?style=flat-square
[forks-url]: https://github.com/wx-chevalier/repo/network/members
[stars-shield]: https://img.shields.io/github/stars/wx-chevalier/repo.svg?style=flat-square
[stars-url]: https://github.com/wx-chevalier/repo/stargazers
[issues-shield]: https://img.shields.io/github/issues/wx-chevalier/repo.svg?style=flat-square
[issues-url]: https://github.com/wx-chevalier/repo/issues
[license-shield]: https://img.shields.io/github/license/wx-chevalier/repo.svg?style=flat-square
[license-url]: https://github.com/wx-chevalier/repo/blob/master/LICENSE.txt
