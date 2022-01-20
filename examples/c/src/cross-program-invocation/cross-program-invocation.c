/**
 * @brief A program demonstrating cross program invocations
 */
#include <paychains_sdk.h>

/// Amount of bytes of account data to allocate
#define SIZE 42

extern uint64_t do_invoke(PayParameters *params) {
  // As part of the program specification the first account is the system
  // program's executable account and the second is the account to allocate
  if (params->ka_num != 2) {
    return ERROR_NOT_ENOUGH_ACCOUNT_KEYS;
  }
  PayAccountInfo *system_program_info = &params->ka[0];
  PayAccountInfo *allocated_info = &params->ka[1];

  uint8_t seed[] = {'Y', 'o', 'u', ' ', 'p', 'a', 's', 's',
                    ' ', 'b', 'u', 't', 't', 'e', 'r'};
  const PaySignerSeed seeds[] = {{seed, PAY_ARRAY_SIZE(seed)},
                                 {&params->data[0], 1}};
  const PaySignerSeeds signers_seeds[] = {{seeds, PAY_ARRAY_SIZE(seeds)}};

  PayPubkey expected_allocated_key;
  if (SUCCESS != pay_create_program_address(seeds, PAY_ARRAY_SIZE(seeds),
                                            params->program_id,
                                            &expected_allocated_key)) {
    return ERROR_INVALID_INSTRUCTION_DATA;
  }
  if (!PayPubkey_same(&expected_allocated_key, allocated_info->key)) {
    return ERROR_INVALID_ARGUMENT;
  }

  PayAccountMeta arguments[] = {{allocated_info->key, true, true}};
  uint8_t data[4 + 8];            // Enough room for the Allocate instruction
  *(uint16_t *)data = 8;          // Allocate instruction enum value
  *(uint64_t *)(data + 4) = SIZE; // Size to allocate
  const PayInstruction instruction = {system_program_info->key, arguments,
                                      PAY_ARRAY_SIZE(arguments), data,
                                      PAY_ARRAY_SIZE(data)};
  return pay_invoke_signed(&instruction, params->ka, params->ka_num,
                           signers_seeds, PAY_ARRAY_SIZE(signers_seeds));
}

extern uint64_t entrypoint(const uint8_t *input) {
  PayAccountInfo accounts[2];
  PayParameters params = (PayParameters){.ka = accounts};

  if (!pay_deserialize(input, &params, PAY_ARRAY_SIZE(accounts))) {
    return ERROR_INVALID_ARGUMENT;
  }

  return do_invoke(&params);
}
