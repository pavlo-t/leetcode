ThisBuild / scalaVersion     := "2.13.6"
ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "com.example"
ThisBuild / organizationName := "example"

lazy val root = (project in file("."))
  .settings(
    name := "LeetCode challenge",
    libraryDependencies += "org.scalatest" %% "scalatest" % "3.2.9"
  )
