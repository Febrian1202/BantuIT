<?php

namespace App\DTOs\Auth;

/**
 * Data Transfer Object for login data.
 */
class LoginData
{
    public function __construct(
        public readonly string $email,
        public readonly string $password,
    ) {}

    public static function fromArray(array $data): self
    {
        return new self($data['email'], $data['password']);
    }
}
