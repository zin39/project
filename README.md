Project Jhappak


-----------------------------------------
-----------------------------------------

Building a fast delivery network for resources .


Core idea : 

	- a customer places an order . (location , resource) 
	- using queue as data strucutre to put tasks in order . 
	- assigning a seperate queue for drivers.
	- checking if there is any driver in the queue if yes than the task assigned to the driver.
	- driver completes the task and joins the queue again.
	- after the task completion removal of the task.

=============================
=============================

First Part :
	- Setting up a server which accepts  messages from clients and runs a certain hooks.
		Hook is a kind of  code which runs when a certain message is recieved.
		For Example when a cusotmer sends a message "HI" . we need to launch a hook which 
		will return hello user. And so with the drivers.


Understanding logic :
	- We have user component . 
		- User which sends Name , Location , Phone number , Quantity
	- We have owner component . 
		- Owner accepts or rejects the request 
		- if accepts than he checks the location variable for strings available 
		  (F,G,H,I) and assigns  Name , Location , Phone number , Quantity to each
		  variable.
		- now if "f" or "F" found than it assigns the request to driver(F) variable.
		- similary stores variables consecutively to driver(G), driver(H), driver(I).
	
	- We have driver component
		- In this component driver(G), driver(H), driver(I), driver(F) are seperate entities 
		  which accepts orders . 
		- now when driver recives any input from owner it waits untill the counter is 3 . 
		- after counter is 3 than it moves to locations 1 , 2 and 3 . 
		- returns back to start.

==========================================
==========================================
Understanding Needs :
	- User Component 
		- needs order status
		- needs payment status
		- needs time of arrival 
		- needs driver number
		- needs driver name 

	- Owner Component 
		- needs Name , Location , Quantity , Floor , Payment History , Resource from User
		- needs drivers Status (nil , free , busy ) , Resource_carrying (not quantity, quantity for user)
		- needs counter to store all requests accepted , delivery failed and delivery_on_way
		- needs to point to particular driver (F,G,H,I) with location , floor , quantity
	
	- Driver Component 
		- needs to accept or reject order recieved from the owner
		- needs to inform user and owner the delivery is succesful 
		- needs to recieve payment from the user
		- needs to see available items in warehouse
		- needs to say available items in inventory
		- needs to say if customer accepted or rejected the order and why
		- needs to get location 

==========================================
==========================================

Available Information : 
		- They use a system where the parameter is "alphabet"_"numeric"_"roman"
		- example (I223 I) for Block I House no : 223 Quantity 1. 
		- They have one ware house where they store all the resources
		- They have three drivers and one owner. If required owner also does the delivery
		- If they have four order from same blocks (i.e F,G,H,I) then they give 4 orders to 
		  same delivery guy and sends him to locations X1,X2,X3 with one more driver 
		- If they have 3 order from same blocks then they give 3 orders to same delivery guy 
		  and sends him to deliver the package.	
		- Here he has provided atleast 4 drivers for each blocks. 
		  Assume ram , shyam , ramesh , suresh. Ram assigned to block F, shyam assigned to block G
		  ramesh assigned to block H, Suresh assigned to block I. 




=========================================
=========================================


For handling user request 

		- We will let it be manual for now so that owner feels comfortable to be engaged.
		- Owner gets all the request = >  He chooses from the template => Fills the required info =>  sends to driver 
		- driver recives the info => chooses a template => sends to the owner.
		- Owner recieves the template (from driver) => enters the information => sends it to user => user accepts or rejects it 
		- if user rejects => fills a template => sends to owner => owner verifies => request complete else also request complete 
