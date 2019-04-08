scalaVersion := "2.12.8"

name := "Dank Scala Project"

libraryDependencies += "org.apache.spark" %% "spark-sql" % "2.4.1"

import scalariform.formatter.preferences._

scalariformPreferences := scalariformPreferences.value
    .setPreference(AlignSingleLineCaseStatements, true)
    .setPreference(DoubleIndentConstructorArguments, true)
    .setPreference(DanglingCloseParenthesis, Preserve)

