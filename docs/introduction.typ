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
#lorem(60)

== In this paper
#lorem(20)

=== Contributions
#lorem(40)

= Related Work
#lorem(500)
