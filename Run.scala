import scala.annotation.tailrec

object Run {
   implicit class IntTimes(x: Int) {
      def times [A](f: =>A): Unit = {
         @tailrec def loop(current: Int): Unit =
         
         if(current > 0){
            f
            loop(current - 1)
         }
         loop(x)
      }
   }
}
