  .global DefaultHandler
  .type DefaultHandler,%function
  .thumb_func
DefaultHandler:
  mrs r0, MSP
  b DefaultHandler_
