fun log_numbers(arg_1: Number, arg_2: Number) {
  log.info(arg_1) // Expect: 100
  log.info(arg_2) // Expect: 0
}

log_numbers(100, 0)
