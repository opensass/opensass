> Hiya 👋!

_So, it's Sunday, homeboy, and I usually escape debates and negative thoughts. But today, I was scouring [dev.to](https://dev.to) and stumbled across a post boldly titled [**"Stop Using AWS."**](https://dev.to/code42cate/stop-using-aws-4eg)_

It wasn't the first time I'd seen a call to drop Amazon Web Services for something "simpler", but this one hit a nerve, not because it was provocative, but because it misrepresents why AWS exists and what problems it solves. So, I decided to sit back, relax, and explain **why you should absolutely keep using AWS**, especially if you're serious about software, scalability, and sustainability.

## 🛑 Stop Using AWS

The post opens with a classic anecdote: a developer builds an MVP with all the AWS bells and whistles, Lambda, API Gateway, Cognito, S3, DynamoDB, and no one uses it. The conclusion? AWS was overkill.

But let's be honest: blaming AWS for a failed product is like blaming your tools when your idea doesn't land. **AWS didn't kill your startup. Lack of validation, poor UX, or zero marketing likely did**. Using a battle-tested infrastructure does not guarantee success, but avoiding it can definitely guarantee future pain.

AWS is **not about scale today, it's about resilience tomorrow**. You can start with one Lambda function and scale to millions of invocations without touching infrastructure again. The same service that powers your early prototype can grow into your production backbone. That's not overengineering, that's smart engineering.

## 🚂 Copying Netflix Architecture Is Silly

A key argument in the post is that copying architectures from giants like Netflix is absurd. But here's the thing: **copying proven solutions is how engineering works**.

Bridges don't get new designs for every crossing. Airplanes don't redesign wings every year. Engineers **borrow from what works**, because **battle-tested architectures reduce risk and improve reliability**.

The Netflix architecture may look complex, but many of its core principles, CI/CD, serverless scaling, stateless compute, IAM-based security, are just **good defaults** in AWS. These patterns exist not because they're cool, but because they **work repeatedly under pressure**.

Reusing success isn't a flaw, it's **strategic acceleration**.

## 🚀 Most Projects Die From Lack of Users, Not Overengineering

The post says most early-stage projects fail due to lack of users, not bad infrastructure. But that's not always true.

In reality, **most promising projects die from a lack of marketing and financial support**. As a solo founder currently building an open source project, I can tell you firsthand: the only reason it's still alive is because the community likes it. Not because of my infrastructure. But if tomorrow it catches fire on Hacker News, I know AWS will keep it alive **without migration, without panic, and without sysadmin duty**.

And let's be real, **bad infrastructure won't get you users, but it will lose them fast**. Downtime, slow loading, broken auth, insecure APIs, these will kill user trust before your next commit. AWS lets you **ship confidently**, even with zero staff and zero budget.

## 🔧 Just Use a VPS + Docker Compose

Ah yes, the $5 VPS dream. The classic hacker manifesto. And while it's a romantic idea, **it quickly collapses under real-world pressure**.

- Who patches your server?

- Who restores it after a reboot?

- Who handles DDoS protection?

- Who backs up your database?

- Who monitors CPU/memory spikes?

- Who gives you rolling deployments?

- Who gives you audit logs?

- Who keeps your secrets safe?

- Who helps during an outage?

When you deploy over SSH and `docker compose up`, **you are your own SRE**. That's fine for side projects. But if you're building anything real, anything you want to monetize, secure, or scale, **you need more than a hobby code viber stack**.

AWS takes these headaches away. You get backup, observability, IAM, autoscaling, and global distribution out of the box. No extra effort. No late-night pager duties.

## 🧠 AWS Is Too Complicated

Yes, AWS can feel complex. It has over 200 services. It has dozens of ways to deploy. It can be overwhelming. But so is the Linux kernel. So is Kubernetes. So is React, honestly.

**Complexity is not a flaw, lack of progressive learning is.**

AWS is built for gradual discovery:

- Start with [Amplify](https://aws.amazon.com/amplify/) or [Lightsail](https://aws.amazon.com/lightsail/).

- Move to [CDK](https://aws.amazon.com/cdk/) or [SAM](https://aws.amazon.com/serverless/sam).

- Add services as you go.

The best part? You **don't have to throw anything away** when you grow. That's the opposite of overengineering. That's **technical maturity baked into the platform**.

You can stay lean and still have optionality for years.

## ✅ When AWS Makes Sense: Always

The post says AWS only makes sense in a few niche scenarios: job hunting, compliance, global scale. That's a narrow view.

Here's when AWS actually makes sense:

- You care about security and compliance from day one.

- You want an upgrade path without rewrite.

- You hate ops and want managed services.

- You care about observability and structured logs.

- You want global latency under 100ms.

- You want fine-grained access control.

- You want infra as code and rollback.

- You want your infra to _just work_.

AWS isn't for everyone. But if you're serious about building something resilient, flexible, and long-lasting, **it's one of the smartest tools you can use.**

## 🦀 Why You Should Use Rust for Everything, Everywhere, Always

There's one more piece of the stack we haven't talked about: the programming language.

If you're still choosing Node.js, Python, Ruby, or Go by default, it's time to stop and seriously consider **Rust**, not just as an option, but as your **primary tool for all stacks**.

Rust is not a trend. It is **a revolution in software correctness, safety, and performance**. The following are some of the reasons why using Rust across your entire stack, CLI tools, web backends, infrastructure tooling, and WebAssembly frontends, [Open SASS](https://github.com/opensass) btw **is the most rational long-term bet you can make**:

- **Memory Safety Without Garbage Collection:** Rust guarantees memory safety at compile time without a GC. You write code that's fast and safe, with zero runtime overhead. This means fewer production crashes, fewer bugs, less stress, and happier developers.

- **Blazing Fast Performance:** Rust consistently outperforms languages like Python, Ruby, and even Go in compute-intensive tasks. It gives you C-level speed with a sane, modern syntax and tooling.

- **First-Class WebAssembly Support:** You can compile Rust to WebAssembly and run it in the browser, or even server-side. This allows **one language to power both frontend and backend**, closing the full-stack loop.

- **Ergonomic Dev Experience:** With tools like `cargo`, `rust-analyzer`, and strong compiler messages, Rust is not just fast, it's a **joy to work with** once you get the hang of it. The ecosystem is mature and growing rapidly.

- **Concurrency Without Fear:** Rust's ownership model makes concurrent programming safer by default. You don't fear multithreading. You embrace it.

- **Perfect for Cloud and Infrastructure:** Major tools like [`firecracker` (AWS Lambda microVMs)](https://firecracker-microvm.github.io/), [`deno`](https://deno.com/), and [`vector.dev`](https://vector.dev) are all written in Rust. It's becoming the de facto language for **next-gen DevOps, cloud infra, and edge computing**.

- **Low-Level Power, High-Level Syntax:** Need to write performant networking code? Cryptographic primitives? OS-level utilities? Rust does it all, and makes it readable and testable.

- **Future-Proof Your Codebase:** Choosing Rust today is betting on stability, safety, and speed for the next 20 years. It's backed by major players, and already replacing legacy C/C++ in critical systems.

Imagine this: a single language that powers your server, builds your CLI, compiles to your frontend, runs your Lambda functions, writes your Terraform replacements, and lets you deploy blazingly fast binaries to any architecture.

That's not just a dream stack.

That's **Rust.**

> If you're already using AWS, adopting Rust makes even more sense. You can build ultra-efficient Lambda functions, optimize EC2 compute, write blazing-fast CLI tools, and reduce cold start times dramatically, all while writing safer code.

The world is slowly migrating to Rust, one subsystem at a time. The only question is: **why not be early?**

## 🔚 Final Thoughts: Your Stack Shouldn't Be a Toy

So no, you don't _need_ AWS. But dismissing it as overkill is shortsighted. A startup doesn't need a Formula 1 car, but it also shouldn't build a go-kart out of duct tape and optimism.

If your product succeeds, you'll wish you had AWS from the start. If it fails, it won't be because you used AWS. It'll be because no one wanted what you built. **Don't blame your infrastructure for your idea's failure.**

Build lean. Build smart. But build with **forward motion**.

**AWS ain't your enemy. It's your parachute.**

> **We are Open SASS, babe!**.

> We're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time 👋!
