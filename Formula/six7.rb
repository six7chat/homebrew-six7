class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.9"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.9"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.9/six7-0.7.9-arm64-apple-darwin.tar.gz"
      sha256 "8a5180dd7743578943123c65eb83502f814ca3c4de58a66d7e380ffbd40e2ab4"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
