from typing import Tuple

class Dilithium2:
    """Post-quantum digital signature scheme Dilithium2 (NIST Security Level 1).
    
    Provides quantum-resistant signature operations using lattice-based cryptography.
    All keys and signatures are Base64-encoded strings for compatibility.
    """
    
    @staticmethod
    def generate_keypair() -> Tuple[str, str]:
        """Generates a new key pair.
        
        Returns:
            Tuple[str, str]: 
                - Public key (Base64-encoded)
                - Private key (Base64-encoded)
        
        Example:
            >>> pk, sk = Dilithium2.generate_keypair()
            >>> len(pk) > 100
            True
        """
        ...
    
    @staticmethod
    def sign_message(message: str, private_key: str) -> str:
        """Signs a message with the private key.
        
        Args:
            message: Plaintext string to sign
            private_key: Base64-encoded private key from generate_keypair()
            
        Returns:
            str: Base64-encoded signature
            
        Raises:
            ValueError: If private_key format is invalid
        """
        ...
    
    @staticmethod
    def verify_message(signature: str, message: str, public_key: str) -> bool:
        """Verifies a message signature.
        
        Args:
            signature: Base64-encoded signature from sign_message()
            message: Original plaintext message
            public_key: Base64-encoded public key from generate_keypair()
            
        Returns:
            bool: True if signature is valid, False otherwise
            
        Raises:
            ValueError: If signature or public_key format is invalid
        """
        ...

class Dilithium3:
    """Post-quantum digital signature scheme Dilithium3 (NIST Security Level 2).
    
    Provides stronger security guarantees than Dilithium2 with larger key sizes.
    All keys and signatures are Base64-encoded strings for compatibility.
    """
    
    @staticmethod
    def generate_keypair() -> Tuple[str, str]:
        """Generates a new key pair. See Dilithium2.generate_keypair() for details."""
        ...
    
    @staticmethod
    def sign_message(message: str, private_key: str) -> str:
        """Signs a message with the private key. See Dilithium2.sign_message() for details."""
        ...
    
    @staticmethod
    def verify_message(signature: str, message: str, public_key: str) -> bool:
        """Verifies a message signature. See Dilithium2.verify_message() for details."""
        ...

class Dilithium5:
    """Post-quantum digital signature scheme Dilithium5 (NIST Security Level 3).
    
    Provides the highest security level with largest key sizes.
    All keys and signatures are Base64-encoded strings for compatibility.
    """
    
    @staticmethod
    def generate_keypair() -> Tuple[str, str]:
        """Generates a new key pair. See Dilithium2.generate_keypair() for details."""
        ...
    
    @staticmethod
    def sign_message(message: str, private_key: str) -> str:
        """Signs a message with the private key. See Dilithium2.sign_message() for details."""
        ...
    
    @staticmethod
    def verify_message(signature: str, message: str, public_key: str) -> bool:
        """Verifies a message signature. See Dilithium2.verify_message() for details."""
        ...