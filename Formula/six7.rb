class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.14"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.14"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.14/six7-0.7.14-arm64-apple-darwin.tar.gz"
      sha256 "cb4bd383467889d74735ebf959fff702ab0a7813907039e1bb5c4c572e9c6fd0"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
