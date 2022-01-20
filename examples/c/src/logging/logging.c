/**
 * @brief A program demonstrating logging
 */
#include <paychains_sdk.h>

extern uint64_t logging(PayParameters *params) {
  // Log a string
  pay_log("static string");

  // Log 5 numbers as u64s in hexadecimal format
  pay_log_64(params->data[0], params->data[1], params->data[2], params->data[3],
             params->data[4]);

  // Log a slice
  pay_log_array(params->data, params->data_len);

  // Log a public key
  pay_log_pubkey(params->program_id);

  // Log all the program's input parameters
  pay_log_params(params);

  // Log the number of compute units remaining that the program can consume.
  pay_log_compute_units();

  return SUCCESS;
}

extern uint64_t entrypoint(const uint8_t *input) {
  PayAccountInfo accounts[1];
  PayParameters params = (PayParameters){.ka = accounts};

  if (!pay_deserialize(input, &params, PAY_ARRAY_SIZE(accounts))) {
    return ERROR_INVALID_ARGUMENT;
  }

  return logging(&params);
}
