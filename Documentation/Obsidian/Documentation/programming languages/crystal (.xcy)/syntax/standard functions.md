# Inbuilt traits

this page will show some methods for different types and implementations for simpler methods

#### methods for INT (integer):
- $inc() 
	- increments the number before it by either 1, if no parameters or by the number passed as a parameter 
	- usage:   x.$inc()    // increases variable x by 1 
	- usage:   x.$inc(-3)  // decreases variable x by 3
	- implementation:
			
		function $inc(INT num):
			if : num.$none() :
				@self += 1
				return
			else :
				@self += num
				return
	
- $neg()
	- negates the number it is called on
	- usage: x.$neg()  // x is negated, for example if x is 4, it is set to -4.
	- implementation:

	function $neg():
		@self -= (2 * @self)
		return

- $float()
	- converts an integer type to a floating point type. (this also changes how it is stored in RAM)
	- 