#import "template.typ": *

// Initialize the project
#show: project.with(
  title: "Introduction",
  authors: (
    "Max Fehlinger",
    "Foxx Pinkerton",
  ),
  date: datetime.today().display("[month repr:long] [day], [year]"),
)

= Introduction
Welcome to the introduction of the Winnipeg Transit App! In this overview, we will discuss the
scenario, the rationale for the proposed solution, the technologies used, and highlight the
project's focus on providing a practical learning experience within the given constraints.

= The Scenario
The Winnipeg Transit App is a project developed as part of the computer science 40S course. While
there  is no direct client or advisor involvement from the Winnipeg Department of Transportation,
the app aims to cater to the needs of the general public of Winnipeg, who rely on the city's
transit system for their daily commute. The project's main motivation came from the official app
being slow and unresponsive.

The project's primary goal is to develop a mobile application that provides transit information,
such as real-time bus tracking (if possible), trip planning (and routing notifications), fare
details, and service alerts, to enhance the overall transit experience for users, with a focus on
creating a user-friendly and efficient application.

= The Rationale for the Proposed Solution
The proposed solution for `Transit Trail` is based on personal dissatisfaction with the official
app, due to slow loading times, the GPS positioning and bus tracking working unreliably (sometimes
up to 10 minutes off). While *there were no consultations with Winnipeg Transit*, the project uses
a provided API. This API allows the app to retrieve real-time transit information, ensuring users
can access accurate and up-to-date data.

As the timeframe is only a few weeks/months, we developed this application as a means to improve the
understanding of technologies, within a resonable scope. This means that features have to be
prioritized and might not be realized.

= The Technologies Used
There were three main technologies used:

- The Rust programming language: Rust is memory safe due to it's ownership of data model, which
  makes it compete with other low-level languages, such as C and C++.
- For the frontend, svelte is used in combination with Typescript and Sass. Svelte is an easy-to-use
  JavaScript framework, with no runtime dependencies, decreasing the bundle size significantly,
  compared to more popular frameworks, such as React. To decrease errors in the application,
  Typescript provides a Type System on top of Javascript to ensure, no unsafe accesses are made.
  Sass is an extension of CSS and adds easy variable use to it, improving the development experience
  greatly.
- Both technologies are combined with Tauri. Tauri is a relatively new project to build
  applications - currently only for the desktop, but mobile application development is in the alpha
  phase - with Rust as the backend language and any web framework for the frontend. It aims to be a
  competitor to electronJS, but with a smaller memory footprint, by utilizing native rendering
  technologies, in place of a chromium instance.

The required software needed for this project is available for free (VSCode, recommended by Tauri)
or through the GitHub student development package. All libraries and frameworks used are licensed
under an open-source license and therefore also freely available. While most technologies are
different and relatively new to the development world, knowledge about these tools is available in
part and the rest has to be learned through the official documentation or tutorials.

= Resolving Security Implications
Due to the small timeframe, we are not focusing on extensive cybersecurity measures, regular safety
checks, or reviews through external sources. However, it's essential to
acknowledge the significance of security and privacy in software applications. To implement this, we
will do our best to always include the latest versions of the libraries used and use Rust in the
backend to prevent common memory leaks in other programming languages. We will follow
best practices for coding securely, such as implementing specific authentication mechanisms,
protecting against common vulnerabilities, and handling user data responsibly within the project's
scope.

= Summary
`Transit Trail` serves as a practical learning experience for
developing a real-world application with access to outside real-time information. We aim to
leverage the benefits from the technologies used, utilize the available tools effectively, and
prioritize user experience while considering security principles within the project's constraints.

