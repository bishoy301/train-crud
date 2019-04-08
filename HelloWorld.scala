import org.apache.spark.sql.DataFrame
object HelloWorld {
   /* This is my first java program.  
    * This will print 'Hello World' as the output
    * This is an example of multi-line comments.
    */
   def main(args: Array[String]) {
      // Prints Hello World
      // This is also an example of single line comment.
      var name = "Chihiro Fujisaki";
      val greeting = new StringBuilder
      greeting ++= "Hello World "
      greeting ++= name
      println(greeting)
   }
}
