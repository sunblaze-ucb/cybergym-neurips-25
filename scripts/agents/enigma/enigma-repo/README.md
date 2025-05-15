<p align="center">
  <a href="https://www.swe-agent.com/">
    <img src="assets/swe-agent-banner.png" alt="swe-agent.com" />
  </a>
</p>

<p align="center">
  <a href="#enigma"><img src="https://github.com/user-attachments/assets/70ba3fdf-ee7b-474f-8fdc-de4ffde51eef" height="35px"></a>
</p>

<p align="center">
  <a href="https://swe-agent.com"><strong>Documentation</strong></a>&nbsp; | &nbsp;
  <a href="https://discord.gg/AVEFbBn2rH"><strong>Discord</strong></a>&nbsp; | &nbsp;
  <a href="https://arxiv.org/abs/2405.15793"><strong>Paper</strong></a>&nbsp; | &nbsp;
  <a href="https://arxiv.org/abs/2409.16165"><strong>EnIGMA preprint</strong></a>
</p>

**SWE-agent turns LMs (e.g. GPT-4) into software engineering agents that can resolve issues in real GitHub repositories and more.**

On [SWE-bench][], SWE-agent resolves 12.47% of issues of the full test set and 23% of issues of SWE-bench lite.
[SWE-agent EnIGMA][enigma] solves more than **3x more** challenges of the offensive cybersecurity [NYU CTF benchmark][nyu-ctf] than the previous SOTA agent.

[enigma]: https://enigma-agent.com
[SWE-bench]: https://github.com/swe-agent/SWE-bench
[nyu-ctf]: https://arxiv.org/abs/2406.05590

We accomplish our results by designing simple LM-centric commands and feedback formats to make it easier for the LM to browse the repository, view, edit and execute code files. We call this an **Agent-Computer Interface (ACI)**.
Read more about it in our [paper](https://arxiv.org/abs/2405.15793)!

SWE-agent is built and maintained by researchers from Princeton University.

![swe-agent-gui-demo](https://github.com/swe-agent/SWE-agent/assets/13602468/fa201621-ec31-4644-b658-c1d0feb92253)

## 🚀 Get started!

👉 Try SWE-agent in your browser: [![Open in GitHub Codespaces](https://img.shields.io/badge/Open_in_GitHub_Codespaces-gray?logo=github)](https://codespaces.new/SWE-agent/SWE-agent) ([more information](https://swe-agent.com/latest/installation/codespaces/))

Read our [documentation][docs] to learn more:

* [Installation](https://swe-agent.com/latest/installation/)
* [Command line usage](https://swe-agent.com/latest/usage/cl_tutorial/)
* [Using the web UI](https://swe-agent.com/latest/usage/web_ui/)
* [Benchmarking on SWE-bench](https://swe-agent.com/latest/usage/benchmarking/)
* [Frequently Asked Questions](https://swe-agent.com/latest/faq/)

Our most recent lecture touches on the project's motivation,
showcases our research findings and provides a hands-on tutorial on how to install,
use, and configure SWE-agent:

<div align="center">
<a href="https://youtu.be/d9gcXpiiDao"><img src="assets/wb_stream_youtube.png" style="width: 600px"/></a>
</div>

[docs]: https://swe-agent.com

## 🕵️ SWE-agent for offensive cybersecurity (EnIGMA) <a name="enigma"></a>
<img src="https://github.com/user-attachments/assets/5128cc06-7a28-4a37-b950-e4e58bc00823" height="80px"></img>

[SWE-agent: EnIGMA][enigma] is a mode for solving offensive cybersecurity (capture the flag) challenges.
EnIGMA achieves state-of-the-art results on multiple cybersecurity benchmarks (see [leaderboard](https://enigma-agent.com/#results)).
The EnIGMA project introduced multiple features that are available in all modes of SWE-agent, such as the [debugger and server connection tools](https://swe-agent.com/0.7/background/iat/) and a [summarizer](https://swe-agent.com/0.7/config/summarizers/) to handle long outputs.

<img src="https://github.com/user-attachments/assets/a3bdcc06-9193-4368-b612-c7cf94a9482c" height="200px"></img>


## 💫 Contributions <a name="contributions"></a>
- If you'd like to ask questions, learn about upcoming features, and participate in future development, join our [Discord community](https://discord.gg/AVEFbBn2rH)!
- If you'd like to contribute to the codebase, we welcome [issues](https://github.com/swe-agent/SWE-agent/issues) and [pull requests](https://github.com/swe-agent/SWE-agent/pulls)!

Contact person: [John Yang](https://john-b-yang.github.io/) and [Carlos E. Jimenez](http://www.carlosejimenez.com/) (Email: johnby@stanford.edu, carlosej@princeton.edu).

## 📝 Citation <a name="citation"></a>
If you found this work helpful, please consider citing it using the following:
```bibtex
@misc{yang2024sweagent,
      title={SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering},
      author={John Yang and Carlos E. Jimenez and Alexander Wettig and Kilian Lieret and Shunyu Yao and Karthik Narasimhan and Ofir Press},
      year={2024},
      eprint={2405.15793},
      archivePrefix={arXiv},
      primaryClass={cs.SE}
}
```

If you used the summarizer, interactive commands or the offensive cybersecurity capabilities in SWE-agent, please also consider citing:
```bibtex
@misc{abramovich2024enigmaenhancedinteractivegenerative,
      title={EnIGMA: Enhanced Interactive Generative Model Agent for CTF Challenges},
      author={Talor Abramovich and Meet Udeshi and Minghao Shao and Kilian Lieret and Haoran Xi and Kimberly Milner and Sofija Jancheska and John Yang and Carlos E. Jimenez and Farshad Khorrami and Prashanth Krishnamurthy and Brendan Dolan-Gavitt and Muhammad Shafique and Karthik Narasimhan and Ramesh Karri and Ofir Press},
      year={2024},
      eprint={2409.16165},
      archivePrefix={arXiv},
      primaryClass={cs.AI},
      url={https://arxiv.org/abs/2409.16165},
}
```


## 🪪 License <a name="license"></a>
MIT. Check `LICENSE`.

<div align="center">

[![Pytest](https://github.com/swe-agent/SWE-agent/actions/workflows/pytest.yaml/badge.svg)](https://github.com/swe-agent/SWE-agent/actions/workflows/pytest.yaml)
[![Test build containers](https://github.com/swe-agent/SWE-agent/actions/workflows/test_build_containers.yaml/badge.svg)](https://github.com/swe-agent/SWE-agent/actions/workflows/test_build_containers.yaml)
[![Release to dockerhub (nightly)](https://github.com/swe-agent/SWE-agent/actions/workflows/release-dockerhub-nightly.yaml/badge.svg)](https://github.com/swe-agent/SWE-agent/actions/workflows/release-dockerhub-nightly.yaml)
[![Release to dockerhub (release)](https://github.com/swe-agent/SWE-agent/actions/workflows/release-dockerhub-release.yaml/badge.svg)](https://github.com/swe-agent/SWE-agent/actions/workflows/release-dockerhub-release.yaml)
[![build-docs](https://github.com/swe-agent/SWE-agent/actions/workflows/build-docs.yaml/badge.svg)](https://github.com/swe-agent/SWE-agent/actions/workflows/build-docs.yaml)
[![codecov](https://codecov.io/gh/SWE-agent/SWE-agent/graph/badge.svg?token=18XAVDK365)](https://codecov.io/gh/SWE-agent/SWE-agent)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/SWE-agent/SWE-agent/main.svg)](https://results.pre-commit.ci/latest/github/SWE-agent/SWE-agent/main)
[![Markdown links](https://github.com/swe-agent/SWE-agent/actions/workflows/check-links.yaml/badge.svg)](https://github.com/swe-agent/SWE-agent/actions/workflows/check-links.yaml)

</div>
